//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 828/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk828<F: Float>(t128: F, t569: F, t1016: F, t142: F, t182: F, t310: F, t495: F, t814: F, t2605: F, t2609: F, t2612: F, t2615: F) -> (F, F, F, F, F, F, F, F) {
    let t8887 = t569 * t128;
    let t8927 = t142 * t1016;
    let t10098 = t310 * t182;
    let t10952 = t814 * t495;
    let t11509 = F::cast_from(0.22787578869697033845e-2_f64) * t2605;
    let t11510 = F::cast_from(0.13780319445925925925e-1_f64) * t2609;
    let t11511 = F::cast_from(0.65061487801810439052e-1_f64) * t2612;
    let t11512 = F::cast_from(0.19263893255070628431e1_f64) * t2615;
    (t8887, t8927, t10098, t10952, t11509, t11510, t11511, t11512)
}
