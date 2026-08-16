//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 975/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk975<F: Float>(t14902: F, t10243: F, t10658: F, t14718: F, t14892: F, t14899: F, t15058: F, t15062: F, t15065: F, t15069: F, t15116: F, t15087: F, t15099: F, t15112: F) -> F {
    let t15118 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14902;
    let t15123 = -F::cast_from(22.0_f64) / F::cast_from(27.0_f64) * t14718 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10243 - t14892 / F::cast_from(3.0_f64) - t15116 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14899 + t15118 + t15058 / F::cast_from(6.0_f64) - t10658 - t15062 / F::cast_from(6.0_f64) - t15065 / F::cast_from(12.0_f64) + t15069 / F::cast_from(8.0_f64);
    let t15125 = t15087 + t15099 + t15112 + t15123;
    t15125
}
