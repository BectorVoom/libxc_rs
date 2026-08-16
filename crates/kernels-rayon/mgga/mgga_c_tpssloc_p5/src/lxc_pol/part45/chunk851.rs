//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 851/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk851(t30677: f64, t6637: f64, t6552: f64, t794: f64, t8356: f64, t6562: f64, t1902: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t1894: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30678 = t6637 * t30677;
    let t30680 = 0.3289868133696452873e-1_f64 * t6552 * t30678;
    let t30681 = t794 * t8356;
    let t30683 = 0.82246703342411321825e-2_f64 * t6562 * t30681;
    let t30684 = t1902 * t828;
    let t30685 = t30684 * t232;
    let t30686 = t6646 * t30685;
    let t30688 = 0.16449340668482264365e-1_f64 * t1888 * t30686;
    let t30689 = t1894 * t6624;
    (t30678, t30680, t30681, t30683, t30685, t30686, t30688, t30689)
}
