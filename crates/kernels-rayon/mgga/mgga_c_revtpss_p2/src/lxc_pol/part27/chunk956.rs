//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 956/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk956(t11545: f64, t291: f64, t2942: f64, t941: f64, t11410: f64, t954: f64, t2986: f64, t960: f64, t11467: f64, t973: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11547 = 0.621814e-1_f64 * t11545 * t291;
    let t11548 = t941 * t2942;
    let t11551 = t11410 * t954;
    let t11554 = t960 * t2986;
    let t11557 = t11467 * t973;
    let t11560 = 0.28842592592592592592e-1_f64 * t11132;
    let t11571 = -t11560 - 0.12361111111111111111e-1_f64 * t11134 + 0.61805555555555555556e-2_f64 * t11136 - 0.18541666666666666667e-1_f64 * t11138 + 0.92708333333333333334e-2_f64 * t11140 - 0.10300925925925925926e-1_f64 * t11147 + 0.37083333333333333333e-1_f64 * t11153 - 0.18541666666666666666e-1_f64 * t11158 - 0.55625000000000000001e-1_f64 * t11162 + 0.55625000000000000001e-1_f64 * t11167 - 0.92708333333333333333e-2_f64 * t11171;
    (t11547, t11548, t11551, t11554, t11557, t11571)
}
