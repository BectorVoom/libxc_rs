//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 747/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk747(t70271: f64, t14530: f64, t290: f64, t14580: f64, t899: f64, t70316: f64, t70328: f64, t70376: f64, t70385: f64, t70439: f64, t2228: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71755 = 0.6505345598561924296e-5_f64 * t70271;
    let t71760 = t290 * t14530;
    let t71772 = t899 * t14580;
    let t71775 = 0.6505345598561924296e-5_f64 * t70316;
    let t71789 = 0.3830813990396805546e-3_f64 * t70328;
    let t71802 = 0.162600798888400151e-2_f64 * t70376;
    let t71804 = 0.32526727992809621482e-4_f64 * t70385;
    let t71832 = 0.2316441583394736328e-4_f64 * t70439;
    let t71835 = t2228 * t265;
    (t71755, t71760, t71772, t71775, t71789, t71802, t71804, t71832, t71835)
}
