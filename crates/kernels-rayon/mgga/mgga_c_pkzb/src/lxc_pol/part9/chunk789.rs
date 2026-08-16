//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 789/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk789(t2082: f64, t775: f64, t2065: f64, t771: f64, t1485: f64, t178: f64, t301: f64, t299: f64, t1843: f64, t655: f64, t779: f64, t2888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5607 = t2082 * t775;
    let t5609 = t771 * t2065;
    let t5612 = t178 * t1485 * t301;
    let t5614 = 0.63517063878621832551e-4_f64 * t299 * t5612;
    let t5616 = t779 * t1843 * t655;
    let t5617 = t2888 * t5616;
    (t5607, t5609, t5612, t5614, t5616, t5617)
}
