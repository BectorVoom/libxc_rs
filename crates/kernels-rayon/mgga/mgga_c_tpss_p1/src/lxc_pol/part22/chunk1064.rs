//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1064/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1064(t1407: f64, t2732: f64, t2741: f64, t11491: f64, t3933: f64, t3931: f64, t1460: f64, t672: f64, t925: f64, t140: f64, t3927: f64, t1465: f64, t2465: f64) -> (f64, f64, f64, f64, f64) {
    let t11630 = t1407 * t2732;
    let t11631 = t2741 * t11630;
    let t11636 = t11491 * t3933;
    let t11637 = t3931 * t11636;
    let t11640 = t672 * t1460;
    let t11641 = t925 * t11640;
    let t11645 = t140 * t3927;
    let t11647 = t925 * t11645 / 432.0_f64;
    let t11648 = t1465 * t2465;
    (t11631, t11637, t11641, t11647, t11648)
}
