//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1026/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1026<F: Float>(t224: F, t42919: F, t43356: F, t44213: F, t44247: F, t13247: F, t41573: F, t41574: F, t41575: F, t41577: F, t41579: F, t41581: F, t41582: F, t41584: F, t41585: F, t41586: F, t42467: F, t42470: F, t42473: F, t42475: F, t42478: F, t42481: F, t42483: F, t42485: F, t856: F) -> (F, F) {
    let t44250 = t224 * (t42919 + t43356 + t44213 + t44247);
    let t51210 = t13247 * t856 + t41573 - t41574 - t41575 - t41577 - t41579 + t41581 - t41582 + t41584 - t41585 - t41586 - t42467 - t42470 - t42473 + t42475 + t42478 - t42481 + t42483 - t42485;
    (t44250, t51210)
}
