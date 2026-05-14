//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 800/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk800<F: Float>(t34013: F, t977: F, t3073: F, t9767: F, t13238: F, t5552: F, t41573: F, t41574: F, t41575: F, t41577: F, t41579: F, t41581: F, t41582: F, t41584: F, t41585: F, t41586: F, t42467: F, t42906: F, t42908: F, t42910: F, t42912: F) -> (F,) {
    let t42914 = t34013 * t977;
    let t42916 = t9767 * t3073;
    let t42917 = t5552 * t13238;
    let t42919 = t42906 - t41573 + t41574 + t41575 + t41577 + t41579 - t41581 + t41582 - t42908 - t41584 + t42910 - 12.0 * t42912 - 2.0 * t42914 - t42916 + t41585 + 4.0 * t42917 + t41586 + t42467;
    (t42919,)
}
