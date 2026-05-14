//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 602/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk602<F: Float>(t2030: F, t549: F, t554: F, t2057: F, t538: F, t138: F, t8153: F, t8157: F, t550: F, t2044: F, t7853: F, t131: F, t1991: F, t139: F, t548: F, t135: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8865 = t549 * t2030;
    let t8866 = t8865 * t554;
    let t8869 = t2057 * t538;
    let t8873 = t138 * t8153;
    let t8874 = t8873 * t8157;
    let t8877 = t2057 * t554;
    let t8883 = t550 * t538;
    let t8885 = t7853 * t2044;
    let t8894 = t1991 * t131;
    let t8895 = t8894 * t139;
    let t8906 = t548 * t548;
    let t8907 = 1.0 / t8906;
    let t8908 = t135 * t8907;
    (t8865, t8866, t8869, t8873, t8874, t8877, t8883, t8885, t8894, t8895, t8906, t8907, t8908)
}
