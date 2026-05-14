//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1230/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1230<F: Float>(t11975: F, t27122: F, t3116: F, t2586: F, t8956: F, t1133: F, t26255: F, t8950: F, t22035: F, t894: F, t3147: F, t7878: F, t8517: F, t1121: F, t3128: F, t3137: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27124 = t3116 * t11975 * t27122;
    let t27126 = t2586 * t8956;
    let t27127 = t1133 * t27126;
    let t27129 = t8950 * t26255;
    let t27131 = t894 * t27129 * t22035;
    let t27134 = t7878 * t3147;
    let t27135 = t1133 * t27134;
    let t27137 = t2586 * t8517;
    let t27138 = t1133 * t27137;
    let t27141 = t1121 * t3137 * t3128;
    (t27124, t27126, t27127, t27131, t27134, t27135, t27137, t27138, t27141)
}
