//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1261/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1261(t1490: f64, t303: f64, t98607: f64, t2237: f64, t27342: f64, t28373: f64, t3801: f64, t3984: f64, t7908: f64, t8151: f64, t94626: f64, t98235: f64, t98361: f64, t98515: f64, t98587: f64, t98593: f64, t98598: f64, t98600: f64, t98604: f64) -> (f64, f64) {
    let t98609 = t303 * t98607 * t1490;
    let t98611 = -0.46336805555555555556e-3_f64 * t7908 * t3984 * t28373 * t3801 + t98587 + 0.27802083333333333334e-2_f64 * t7908 * t98515 - 0.92673611111111111112e-3_f64 * t94626 * t98361 + 0.66327777777777777776e-2_f64 * t98593 + 0.61782407407407407408e-3_f64 * t94626 * t98235 - t98598 + 0.69505208333333333333e-3_f64 * t2237 * t98600 - t98604 + 0.37069444444444444444e-2_f64 * t8151 * t27342 - 0.49745833333333333332e-2_f64 * t98609;
    (t98609, t98611)
}
