//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1242/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1242(t1307: f64, t1842: f64, t1527: f64, t776: f64, t671: f64, t7982: f64, t2169: f64, t214: f64, t6624: f64, t30657: f64, t6547: f64, t30671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    let t112667 = t6547 * t30657;
    let t112673 = t6547 * t30671;
    (t97721, t98960, t104977, t105108, t112660, t112667, t112673)
}
