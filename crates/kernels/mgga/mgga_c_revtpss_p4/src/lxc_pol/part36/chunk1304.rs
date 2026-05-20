//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1304/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1304<F: Float>(t23336: F, t27261: F, t23323: F, t25270: F, t106006: F, t106010: F, t106014: F, t106022: F, t113171: F, t113173: F, t113177: F, t113180: F, t113182: F, t113184: F, t92976: F, t98964: F) -> F {
    let t113186 = t27261 * t23336;
    let t113188 = t25270 * t23323;
    let t113192 = F::cast_from(0.51448821741683684367e-2_f64) * t113171 - F::cast_from(0.12862205435420921092e-2_f64) * t113173 + F::cast_from(0.6098400337114239387e-3_f64) * t106006 - F::cast_from(0.48018900292238105409e-1_f64) * t106010 - F::cast_from(0.17149607247227894789e-2_f64) * t113177 + F::cast_from(0.24009450146119052704e-1_f64) * t106014 - F::cast_from(0.25724410870841842184e-1_f64) * t113180 + F::cast_from(0.51448821741683684367e-2_f64) * t113182 + F::cast_from(0.25724410870841842183e-2_f64) * t113184 - F::cast_from(0.10289764348336736873e-1_f64) * t113186 + F::cast_from(0.51448821741683684367e-2_f64) * t113188 - F::cast_from(0.45732285992607719437e-3_f64) * t98964 + t92976 + F::cast_from(0.15246000842785598468e-2_f64) * t106022;
    t113192
}
