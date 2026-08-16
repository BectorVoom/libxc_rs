//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 939/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk939(t30689: f64, t6562: f64, t794: f64, t22690: f64, t23171: f64, t30676: f64, t112976: f64, t1888: f64, t232: f64, t6646: f64, t82034: f64, t6624: f64, t828: f64) -> (f64, f64, f64, f64, f64) {
    let t112997 = t6562 * t794 * t30689;
    let t112998 = 0.16449340668482264365e-1_f64 * t112997;
    let t113005 = 0.16449340668482264365e-1_f64 * t23171 * t22690 * t30676;
    let t113009 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t112976 * t232;
    let t113023 = 0.16449340668482264365e-1_f64 * t1888 * t6646 * t82034 * t232;
    let t113032 = 0.3289868133696452873e-1_f64 * t1888 * t6646 * t6624 * t828 * t232;
    (t112998, t113005, t113009, t113023, t113032)
}
