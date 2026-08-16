//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1017/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1017<F: Float>(t1928: F, t6785: F, t6931: F, t2030: F, t6782: F, t1948: F, t6936: F, t2034: F, t2080: F, t2093: F, t654: F, t6904: F) -> (F, F, F, F, F, F, F) {
    let t22218 = t6785 * t1928;
    let t22219 = t6931 * t22218;
    let t22222 = t2030 * t6782;
    let t22224 = t6936 * t1948;
    let t22225 = t2034 * t22224;
    let t22228 = t2080 * t2093;
    let t22230 = t654 * t6904;
    (t22218, t22219, t22222, t22224, t22225, t22228, t22230)
}
