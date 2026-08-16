//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2543/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2543(t11821: f64, t65: f64, t11144: f64, t11970: f64, t1660: f64, t27527: f64, t2852: f64, t11150: f64, t27531: f64, t127: f64, t4823: f64, t15690: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53321 = t65 * t11821;
    let t53322 = t53321 * t11144;
    let t53326 = t1660 * t11970;
    let t53328 = t27527 * t2852;
    let t53332 = t27531 * t11150;
    let t53391 = t127 * t4823;
    let t53405 = t127 * t15690;
    (t53322, t53326, t53328, t53332, t53391, t53405)
}
