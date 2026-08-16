//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 898/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk898(t8660: f64, t2529: f64, t844: f64, t269: f64, t2532: f64, t284: f64, t2480: f64, t841: f64, t2617: f64, t894: f64, t2620: f64, t317: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8687 = 28.0_f64 / 27.0_f64 * t8660;
    let t8709 = 1.0_f64 / t2529 / t844;
    let t8710 = t269 * t8709;
    let t8712 = 1.0_f64 / t2532 / t284;
    let t8723 = 0.55403703703703703703e-1_f64 * t8660;
    let t8737 = t841 * t2480;
    let t8749 = 1.0_f64 / t2617 / t894;
    let t8752 = 1.0_f64 / t2620 / t317;
    (t8687, t8710, t8712, t8723, t8737, t8749, t8752)
}
