//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 733/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk733<F: Float>(t14902: F, t1240: F, t2842: F, t4239: F, t870: F, t1250: F, t8232: F, t1882: F, t4164: F, t4169: F, t12001: F, t4159: F) -> (F, F, F, F, F, F, F) {
    let t15118 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14902;
    let t15128 = t1240 * t2842;
    let t15133 = t4239 * t870;
    let t15147 = t8232 * t1250;
    let t15168 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1882 * t4164;
    let t15170 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t4169;
    let t15180 = t12001 * t4159;
    (t15118, t15128, t15133, t15147, t15168, t15170, t15180)
}
