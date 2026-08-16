//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1015/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1015(t1377: f64, t14479: f64, t47071: f64, t14521: f64, t41574: f64, t41575: f64, t41579: f64, t41581: f64, t41585: f64, t41586: f64, t42470: f64, t42473: f64, t42475: f64, t42481: f64, t42483: f64, t42485: f64, t42487: f64, t42491: f64, t42494: f64, t50808: f64, t617: f64) -> (f64, f64, f64) {
    let t50809 = t1377 * t14479;
    let t50811 = 2.0_f64 * t47071;
    let t50812 = t14521 * t617 - t41574 - t41575 - t41579 + t41581 - t41585 - t41586 - t42470 - t42473 + t42475 - t42481 + t42483 - t42485 + t42487 + t42491 + t42494 + t50808 - t50809 - t50811;
    (t50809, t50811, t50812)
}
