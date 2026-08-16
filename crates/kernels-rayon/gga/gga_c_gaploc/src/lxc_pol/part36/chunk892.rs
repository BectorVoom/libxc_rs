//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 892/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk892(t12856: f64, t17288: f64, t2801: f64, t31428: f64, t41573: f64, t41574: f64, t41575: f64, t41577: f64, t41579: f64, t41581: f64, t41582: f64, t41584: f64, t41585: f64, t41586: f64, t42467: f64, t42470: f64, t42473: f64, t42475: f64, t42478: f64, t42481: f64, t42483: f64) -> (f64, f64, f64) {
    let t42485 = 6.0_f64 * t17288 * t12856;
    let t42487 = 2.0_f64 * t31428 * t2801;
    let t42488 = t41573 - t41574 - t41575 - t41577 - t41579 + t41581 - t41582 + t41584 - t41585 - t41586 - t42467 - t42470 - t42473 + t42475 + t42478 - t42481 + t42483 - t42485 + t42487;
    (t42485, t42487, t42488)
}
