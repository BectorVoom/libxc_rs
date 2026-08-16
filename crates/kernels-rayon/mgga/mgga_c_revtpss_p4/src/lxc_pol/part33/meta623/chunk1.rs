//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2063/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2063(t93317: f64, t98852: f64, t2439: f64, t7774: f64, t93170: f64, t25304: f64, t27212: f64, t25301: f64, t93371: f64, t27286: f64, t689: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98856 = 0.15421710918628844644e0_f64 * t93317 * t98852;
    let t98857 = t7774 * t2439;
    let t98858 = t93170 * t98857;
    let t98867 = t25304 * t27212;
    let t98868 = t98867 * t25301;
    let t98875 = t93371 * t98857;
    let t98877 = t27286 * t689;
    let t98879 = 0.14456046980341999104e-1_f64 * t25431 * t98877;
    (t98856, t98858, t98868, t98875, t98877, t98879)
}
