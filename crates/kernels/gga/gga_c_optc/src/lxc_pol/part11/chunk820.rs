//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 820/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk820<F: Float>(t3061: F, t5218: F, t5202: F, t8700: F, t2976: F, t5154: F, t5122: F, t8850: F, t1085: F, t5197: F, t1066: F, t5117: F) -> (F, F, F, F, F, F) {
    let t15374 = t5218 * t3061;
    let t15381 = t5202 * t8700;
    let t15401 = t5154 * t2976;
    let t15408 = t5122 * t8850;
    let t15434 = t5197 * t1085;
    let t15496 = t5117 * t1066;
    (t15374, t15381, t15401, t15408, t15434, t15496)
}
