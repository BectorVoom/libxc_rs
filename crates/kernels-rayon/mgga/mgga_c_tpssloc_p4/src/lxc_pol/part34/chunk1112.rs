//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1112/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1112(t23229: f64, t81715: f64, t225: f64, t23228: f64, t6563: f64, t81597: f64, t1882: f64, t81686: f64, t9537: f64, t1883: f64, t82045: f64, t10109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82069 = t81715 * t23229;
    let t82074 = t23228 * t225;
    let t82122 = t81597 * t6563;
    let t82153 = t81686 * t9537 * t1882;
    let t82218 = t82045 * t1883;
    let t82252 = t225 * t10109;
    (t82069, t82074, t82122, t82153, t82218, t82252)
}
