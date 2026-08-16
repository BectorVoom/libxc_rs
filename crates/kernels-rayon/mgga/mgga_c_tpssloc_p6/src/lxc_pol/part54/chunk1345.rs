//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1345/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1345(t32762: f64, t6883: f64, t1985: f64, t214: f64, t225: f64, t26328: f64, t567: f64, t7722: f64, t6907: f64, t32761: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64) {
    let t120532 = t6883 * t32762;
    let t120533 = 0.38381794893125283518e-1_f64 * t120532;
    let t120542 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t26328 * t225 * t567;
    let t120544 = t214 * t7722;
    let t120547 = 0.16449340668482264365e-1_f64 * t1985 * t120544 * t6907;
    let t120550 = t6897 * t794 * t32761;
    (t120533, t120542, t120544, t120547, t120550)
}
