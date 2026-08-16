//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1087/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1087(t14953: f64, t14969: f64, t15862: f64, t1668: f64, t1685: f64, t2604: f64, t2868: f64, t3285: f64, t71607: f64, t71619: f64, t71620: f64, t72: f64, t75762: f64, t75767: f64, t77828: f64, t78271: f64, t78272: f64, t78273: f64, t78275: f64, t78277: f64, t78279: f64, t78280: f64) -> f64 {
    let t80318 = t77828 + t71607 + t72 * t1685 * t3285 - 0.2363e1_f64 * t1668 * t14953 - 0.59871208509319042821e-1_f64 * t2868 * t14969 + t71619 - t71620 - t78271 - t78272 - t78273 + t78275 - t78277 - t78279 - t78280 - 0.59871208509319042821e-1_f64 * t2604 * t15862 - 0.58171619854173713844e-5_f64 * t75762 - t75767;
    t80318
}
