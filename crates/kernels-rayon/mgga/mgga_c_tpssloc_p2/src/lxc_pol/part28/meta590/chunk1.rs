//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1886/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886(t23270: f64, t25038: f64, t25039: f64, t2553: f64, t25040: f64, t82074: f64, t87712: f64, t25193: f64, t81591: f64, t1484: f64, t2249: f64, t4119: f64, t606: f64) -> (f64, f64, f64, f64, f64) {
    let t87924 = t25038 * t23270 * t25039 * t2553;
    let t87927 = t87712 * t82074 * t25040;
    let t87931 = t81591 * t25193;
    let t87953 = t2249 * t1484;
    let t87957 = t606 * t4119;
    (t87924, t87927, t87931, t87953, t87957)
}
