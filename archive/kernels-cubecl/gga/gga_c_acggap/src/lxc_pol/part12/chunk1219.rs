//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1219/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1219<F: Float>(t36331: F, t36333: F, t36349: F, t36351: F, t36353: F, t36355: F, t31843: F, t31845: F, t31847: F, t31849: F, t31851: F, t31855: F, t31857: F, t32955: F, t36335: F, t36344: F, t36347: F, t36358: F) -> F {
    let t37960 = F::cast_from(0.17149607247227894789e-2_f64) * t36331;
    let t37961 = F::cast_from(0.12862205435420921092e-1_f64) * t36333;
    let t37970 = F::cast_from(0.45351183609335988442e-1_f64) * t36349;
    let t37971 = F::cast_from(0.25724410870841842184e-2_f64) * t36351;
    let t37972 = F::cast_from(0.672375e0_f64) * t36353;
    let t37973 = F::cast_from(0.3361875e0_f64) * t36355;
    let t37977 = -t37960 + t37961 - F::cast_from(0.68598428988911579156e-2_f64) * t36335 - t32955 - F::cast_from(0.21437009059034868486e-3_f64) * t31843 + F::cast_from(0.37737710747524982482e-2_f64) * t31845 - F::cast_from(0.56606566121287473723e-2_f64) * t31847 + F::cast_from(0.31448092289604152068e-2_f64) * t31849 + F::cast_from(0.15724046144802076034e-2_f64) * t31851 - F::cast_from(0.62896184579208304137e-2_f64) * t36344 + F::cast_from(0.94344276868812456206e-2_f64) * t36347 - t37970 - t37971 + t37972 + t37973 + F::cast_from(0.68598428988911579156e-2_f64) * t31855 - F::cast_from(0.10289764348336736873e0_f64) * t36358 + F::cast_from(0.13719685797782315831e-1_f64) * t31857;
    t37977
}
