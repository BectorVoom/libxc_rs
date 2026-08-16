//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 796/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk796(t179: f64, t1843: f64, t2068: f64, t299: f64, t2003: f64, t655: f64, t758: f64, t301: f64, t486: f64, t154: f64, t276: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5680 = t179 * t2068 * t1843;
    let t5681 = t299 * t5680;
    let t5683 = t2003 * t655;
    let t5684 = t5683 * t1843;
    let t5685 = t758 * t5684;
    let t5688 = t486 * t301;
    let t5690 = t154 * t5688 * t655;
    let t5691 = t276 * t5690;
    let t5693 = t300 * t2003;
    (t5680, t5681, t5684, t5685, t5688, t5690, t5691, t5693)
}
