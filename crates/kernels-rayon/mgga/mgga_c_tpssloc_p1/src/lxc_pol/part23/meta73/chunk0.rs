//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 438/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk438(t2229: f64, t19: f64, t84: f64, t85: f64, t24: f64, t42: f64, t54: f64, t240: f64, t59: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2230 = 1.0_f64 / t2229;
    let t2232 = 0.9492e2_f64 * t19 * t2230;
    let t2239 = 1.0_f64 / t85 / t84;
    let t2240 = t24 * t2239;
    let t2267 = 1.0_f64 / t42;
    let t2274 = 1.0_f64 / t54;
    let t2281 = t59 * t240;
    (t2230, t2232, t2239, t2240, t2267, t2274, t2281)
}
