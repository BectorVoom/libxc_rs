//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1226/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1226(t25410: f64, t98848: f64, t2439: f64, t7774: f64, t93170: f64, t25304: f64, t27212: f64, t25301: f64, t93371: f64, t1580: f64, t25334: f64, t27253: f64, t9775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98849 = t98848 * t25410;
    let t98857 = t7774 * t2439;
    let t98858 = t93170 * t98857;
    let t98867 = t25304 * t27212;
    let t98868 = t98867 * t25301;
    let t98875 = t93371 * t98857;
    let t98920 = t2439 * t25334 * t1580;
    let t98964 = t9775 * t27253;
    (t98849, t98858, t98868, t98875, t98920, t98964)
}
