//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 914/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk914(t1519: f64, t794: f64, t28: f64, t40772: f64, t1649: f64, t2752: f64, t1834: f64, t213: f64, t225: f64, t22573: f64, t7684: f64, t2094: f64, t40611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86893 = t794 * t1519;
    let t89953 = t40772 * t28;
    let t89992 = t2752 * t1649;
    let t90544 = t794 * t1834;
    let t90566 = t213 * t1834 * t225;
    let t91655 = t7684 * t22573;
    let t92169 = t2094 * t40611;
    (t86893, t89953, t89992, t90544, t90566, t91655, t92169)
}
