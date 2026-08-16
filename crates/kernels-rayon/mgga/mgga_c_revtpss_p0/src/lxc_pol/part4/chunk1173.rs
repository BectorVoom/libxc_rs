//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1173/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1173(t10867: f64, t225: f64, t213: f64, t10871: f64, t2722: f64, t2777: f64, t4518: f64, t2439: f64, t2470: f64, t4499: f64, t2798: f64, t1568: f64, t2783: f64) -> (f64, f64, f64, f64, f64) {
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    let t14547 = t10871 * t2722;
    let t14557 = t2777 * t4518;
    let t14558 = t2439 * t14557;
    let t14563 = t4499 * t2470;
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    (t14546, t14547, t14558, t14564, t14567)
}
