//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1135/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1135<F: Float>(t24785: F, t12782: F, t5211: F, t7106: F, t1820: F, t1885: F, t41432: F, t995: F, t12544: F, t7130: F, t32670: F, t41359: F) -> (F, F, F, F, F, F) {
    let t48092 = F::cast_from(128.0_f64) / F::cast_from(1215.0_f64) * t24785;
    let t48095 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5211 * t7106 * t12782;
    let t48099 = F::cast_from(32.0_f64) / F::cast_from(5.0_f64) * t1820 * t1885 * t41432 * t995;
    let t48101 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7130 * t12544;
    let t48102 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t32670;
    let t48103 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t41359;
    (t48092, t48095, t48099, t48101, t48102, t48103)
}
