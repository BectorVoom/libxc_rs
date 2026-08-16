//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1422/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422(t4522: f64, t874: f64, t9288: f64, t1573: f64, t40317: f64, t10867: f64, t1568: f64, t4503: f64, t786: f64, t40270: f64, t4496: f64, t10115: f64, t1576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51445 = t874 * t4522 * t9288;
    let t51452 = t40317 * t1573;
    let t51498 = t10867 * t1568;
    let t51548 = t4503 * t1568;
    let t51549 = t786 * t51548;
    let t51553 = t40270 * t4496;
    let t51578 = t10115 * t1576;
    (t51445, t51452, t51498, t51549, t51553, t51578)
}
