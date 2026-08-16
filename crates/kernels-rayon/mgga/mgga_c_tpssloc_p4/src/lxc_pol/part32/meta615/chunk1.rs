//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2017/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2017(t82122: f64, t794: f64, t852: f64, t23030: f64, t23208: f64, t1882: f64, t81686: f64, t9537: f64, t213: f64, t225: f64, t6556: f64, t81632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82123 = 0.16220877603642232915e0_f64 * t82122;
    let t82133 = t794 * t852;
    let t82147 = t23030 * t23208;
    let t82153 = t81686 * t9537 * t1882;
    let t82154 = 0.13707783890401886971e-2_f64 * t82153;
    let t82159 = t213 * t852 * t225;
    let t82209 = t81632 * t6556;
    (t82123, t82133, t82147, t82154, t82159, t82209)
}
