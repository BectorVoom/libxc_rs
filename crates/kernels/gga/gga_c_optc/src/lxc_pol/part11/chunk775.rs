//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 775/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk775<F: Float>(t2976: F, t5154: F, t5122: F, t8850: F, t1085: F, t5197: F, t1066: F, t5117: F, t241: F, t3058: F, t5218: F, t5202: F, t8697: F, t2586: F, t5336: F, t1133: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15401 = t5154 * t2976;
    let t15408 = t5122 * t8850;
    let t15434 = t5197 * t1085;
    let t15496 = t5117 * t1066;
    let t15562 = t241 * t5197;
    let t15571 = t3058 * t5218;
    let t15582 = t8697 * t5202;
    let t15597 = t2586 * t5336;
    let t15598 = t1133 * t15597;
    (t15401, t15408, t15434, t15496, t15562, t15571, t15582, t15597, t15598)
}
