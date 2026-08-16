//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1094/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1094(t1382: f64, t13836: f64, t605: f64, t39340: f64, t921: f64, t41586: f64, t42470: f64, t42473: f64, t42475: f64, t42481: f64, t42483: f64, t42485: f64, t42487: f64, t42491: f64, t42494: f64) -> (f64, f64, f64) {
    let t47070 = 2.0_f64 * t1382 * t13836 * t605;
    let t47071 = t39340 * t921;
    let t47072 = t47070 - t41586 - t42470 - t47071 - t42473 + t42475 - t42481 + t42483 - t42485 + t42487 + t42491 + t42494;
    (t47070, t47071, t47072)
}
