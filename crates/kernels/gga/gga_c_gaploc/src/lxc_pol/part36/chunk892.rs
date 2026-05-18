//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 892/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk892<F: Float>(t12856: F, t17288: F, t2801: F, t31428: F, t41573: F, t41574: F, t41575: F, t41577: F, t41579: F, t41581: F, t41582: F, t41584: F, t41585: F, t41586: F, t42467: F, t42470: F, t42473: F, t42475: F, t42478: F, t42481: F, t42483: F) -> (F, F, F) {
    let t42485 = F::new(6.0) * t17288 * t12856;
    let t42487 = F::new(2.0) * t31428 * t2801;
    let t42488 = t41573 - t41574 - t41575 - t41577 - t41579 + t41581 - t41582 + t41584 - t41585 - t41586 - t42467 - t42470 - t42473 + t42475 + t42478 - t42481 + t42483 - t42485 + t42487;
    (t42485, t42487, t42488)
}
