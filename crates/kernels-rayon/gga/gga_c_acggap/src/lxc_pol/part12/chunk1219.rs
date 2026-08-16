//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1219/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1219(t36331: f64, t36333: f64, t36349: f64, t36351: f64, t36353: f64, t36355: f64, t31843: f64, t31845: f64, t31847: f64, t31849: f64, t31851: f64, t31855: f64, t31857: f64, t32955: f64, t36335: f64, t36344: f64, t36347: f64, t36358: f64) -> f64 {
    let t37960 = 0.17149607247227894789e-2_f64 * t36331;
    let t37961 = 0.12862205435420921092e-1_f64 * t36333;
    let t37970 = 0.45351183609335988442e-1_f64 * t36349;
    let t37971 = 0.25724410870841842184e-2_f64 * t36351;
    let t37972 = 0.672375e0_f64 * t36353;
    let t37973 = 0.3361875e0_f64 * t36355;
    let t37977 = -t37960 + t37961 - 0.68598428988911579156e-2_f64 * t36335 - t32955 - 0.21437009059034868486e-3_f64 * t31843 + 0.37737710747524982482e-2_f64 * t31845 - 0.56606566121287473723e-2_f64 * t31847 + 0.31448092289604152068e-2_f64 * t31849 + 0.15724046144802076034e-2_f64 * t31851 - 0.62896184579208304137e-2_f64 * t36344 + 0.94344276868812456206e-2_f64 * t36347 - t37970 - t37971 + t37972 + t37973 + 0.68598428988911579156e-2_f64 * t31855 - 0.10289764348336736873e0_f64 * t36358 + 0.13719685797782315831e-1_f64 * t31857;
    t37977
}
