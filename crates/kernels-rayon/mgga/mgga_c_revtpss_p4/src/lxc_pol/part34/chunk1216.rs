//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1216/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1216(t2435: f64, t27986: f64, t1904: f64, t2439: f64, t25916: f64, t25304: f64, t27883: f64, t25946: f64, t25898: f64, t97699: f64, t2453: f64, t3908: f64, t7911: f64) -> (f64, f64, f64, f64, f64) {
    let t97792 = t2435 * t27986;
    let t97795 = t2439 * t25916 * t1904;
    let t97799 = t25304 * t27883;
    let t97800 = t97799 * t25946;
    let t97802 = t97699 * t25898;
    let t97810 = t2453 * t7911 * t3908;
    (t97792, t97795, t97800, t97802, t97810)
}
