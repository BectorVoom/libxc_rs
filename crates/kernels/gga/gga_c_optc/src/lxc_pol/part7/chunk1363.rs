//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1363/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1363<F: Float>(t1122: F, t1900: F, t3119: F, t11975: F, t3116: F, t2586: F, t8956: F, t1133: F, t26255: F, t8950: F, t22035: F, t894: F) -> (F, F, F, F) {
    let t27122 = t1900 * t1122 * t3119;
    let t27124 = t3116 * t11975 * t27122;
    let t27126 = t2586 * t8956;
    let t27127 = t1133 * t27126;
    let t27129 = t8950 * t26255;
    let t27131 = t894 * t27129 * t22035;
    (t27124, t27126, t27127, t27131)
}
