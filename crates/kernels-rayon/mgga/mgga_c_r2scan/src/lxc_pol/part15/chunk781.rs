//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 781/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk781(t817: f64, t312: f64, t317: f64, t6100: f64, t832: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6659 = t817 * t817;
    let t6660 = 1.0_f64 / t6659;
    let t6661 = t312 * t6660;
    let t6678 = 154.0_f64 / 27.0_f64 * t317 * t6100;
    let t6691 = t832 * t832;
    let t6692 = 1.0_f64 / t6691;
    let t6693 = t325 * t6692;
    (t6659, t6660, t6661, t6678, t6691, t6692, t6693)
}
