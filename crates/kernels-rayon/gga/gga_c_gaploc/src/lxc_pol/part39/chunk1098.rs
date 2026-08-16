//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1098/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1098(t41586: f64, t42470: f64, t42473: f64, t42475: f64, t42481: f64, t42483: f64, t42485: f64, t42917: f64, t43346: f64, t47070: f64, t47071: f64, t3749: f64, t7822: f64) -> (f64, f64) {
    let t47095 = 2.0_f64 * t42917 - t47070 + t41586 + t42470 + t47071 + t42473 + t43346 - t42475 + t42481 - t42483 + t42485;
    let t47096 = t7822 * t3749;
    (t47095, t47096)
}
