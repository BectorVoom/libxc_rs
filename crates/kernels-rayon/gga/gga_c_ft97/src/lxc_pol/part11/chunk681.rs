//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 681/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk681(t9152: f64, t9309: f64, t9400: f64, t9458: f64, t160: f64, t9394: f64, t149: f64, t165: f64, t1953: f64, t2081: f64, t2228: f64, t564: f64, t614: f64, t8788: f64, t8790: f64, t9084: f64, t9149: f64, t9259: f64, t9277: f64, t9289: f64, t9429: f64, t9441: f64) -> (f64, f64, f64) {
    let t9460 = t9152 + t9309 + t9400 + t9458;
    let t9462 = t9394 * t160;
    let t9470 = -t149 * t9460 - t165 * t8788 - 2.0_f64 * t165 * t8790 - t165 * t9084 - 3.0_f64 * t1953 * t614 - 3.0_f64 * t2081 * t614 - 3.0_f64 * t2228 * t564 - 6.0_f64 * t9149 - 2.0_f64 * t9259 + 12.0_f64 * t9277 + 12.0_f64 * t9289 - 6.0_f64 * t9429 - 12.0_f64 * t9441 + 2.0_f64 * t9462;
    (t9460, t9462, t9470)
}
