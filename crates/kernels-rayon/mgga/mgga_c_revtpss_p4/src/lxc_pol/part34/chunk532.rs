//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 532/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk532(t1469: f64, t2852: f64, t2857: f64, t1596: f64, t914: f64, t1600: f64, t2880: f64, t2897: f64, t1606: f64, t698: f64, t1614: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4573 = t2852 * t1469;
    let t4578 = t2857 * t1469;
    let t4590 = t1596 * t914;
    let t4598 = t2880 * t1600;
    let t4614 = t2897 * t1600;
    let t4620 = t698 * t1606;
    let t4647 = t1614 * t945;
    (t4573, t4578, t4590, t4598, t4614, t4620, t4647)
}
