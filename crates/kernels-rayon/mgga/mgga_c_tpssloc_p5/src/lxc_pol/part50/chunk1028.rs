//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1028/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1028(t30681: f64, t6562: f64, t1902: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t1894: f64, t6624: f64, t214: f64, t1880: f64, t814: f64, t8347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30683 = 0.82246703342411321825e-2_f64 * t6562 * t30681;
    let t30684 = t1902 * t828;
    let t30685 = t30684 * t232;
    let t30686 = t6646 * t30685;
    let t30688 = 0.16449340668482264365e-1_f64 * t1888 * t30686;
    let t30689 = t1894 * t6624;
    let t30690 = t214 * t30689;
    let t30692 = 0.16449340668482264365e-1_f64 * t1880 * t30690;
    let t30694 = t814 * t8347;
    (t30683, t30685, t30686, t30688, t30689, t30690, t30692, t30694)
}
