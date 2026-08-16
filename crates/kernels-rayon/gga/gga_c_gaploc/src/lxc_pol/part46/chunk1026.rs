//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1026/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1026(t224: f64, t42919: f64, t43356: f64, t44213: f64, t44247: f64, t13247: f64, t41573: f64, t41574: f64, t41575: f64, t41577: f64, t41579: f64, t41581: f64, t41582: f64, t41584: f64, t41585: f64, t41586: f64, t42467: f64, t42470: f64, t42473: f64, t42475: f64, t42478: f64, t42481: f64, t42483: f64, t42485: f64, t856: f64) -> (f64, f64) {
    let t44250 = t224 * (t42919 + t43356 + t44213 + t44247);
    let t51210 = t13247 * t856 + t41573 - t41574 - t41575 - t41577 - t41579 + t41581 - t41582 + t41584 - t41585 - t41586 - t42467 - t42470 - t42473 + t42475 + t42478 - t42481 + t42483 - t42485;
    (t44250, t51210)
}
