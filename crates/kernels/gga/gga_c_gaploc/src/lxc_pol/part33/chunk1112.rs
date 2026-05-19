//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1112/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1112<F: Float>(t22166: F, t22333: F, t23021: F, t787: F, t9824: F, t10024: F, t1980: F, t7442: F, t2586: F, t4752: F, t10007: F, t1710: F, t825: F, t9438: F) -> (F, F, F, F, F, F) {
    let t29025 = F::cast_from(0.59584149919750711116e-1_f64) * t22166 * t22333;
    let t29030 = t787 * t23021;
    let t29032 = F::cast_from(0.29792074959875355558e-1_f64) * t29030 * t9824;
    let t29035 = F::cast_from(0.17875244975925213335e0_f64) * t1980 * t7442 * t10024;
    let t29052 = t4752 * t2586;
    let t29074 = t825 * t9438 * t10007 * t1710;
    (t29025, t29030, t29032, t29035, t29052, t29074)
}
