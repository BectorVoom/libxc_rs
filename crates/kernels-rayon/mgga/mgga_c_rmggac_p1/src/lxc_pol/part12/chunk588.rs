//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 588/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk588(t7692: f64, t1240: f64, t128: f64, t118: f64, t1986: f64, t1994: f64, t1249: f64, t687: f64, t4685: f64, t681: f64, t4616: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7693 = 0.1064114997332445985e-4_f64 * t7692;
    let t7694 = t128 * t1240;
    let t7695 = t118 * t7694;
    let t7696 = t1986 * t7695;
    let t7697 = t1994 * t7696;
    let t7698 = 0.53205749866622299248e-5_f64 * t7697;
    let t7699 = t1249 * t687;
    let t7700 = 0.19957069503106347607e-1_f64 * t7699;
    let t7701 = t4685 * t681;
    let t7702 = 0.14967802127329760705e-1_f64 * t7701;
    let t7703 = t4616 * t664;
    (t7693, t7696, t7698, t7700, t7702, t7703)
}
