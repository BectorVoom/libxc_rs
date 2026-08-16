//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1185/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1185<F: Float>(t3452: F, t5172: F, t10787: F, t5062: F, t14097: F, t5047: F, t5046: F, t10506: F, t251: F, t14611: F, t5180: F, t15061: F, t15063: F, t15066: F, t15069: F, t15072: F, t15074: F, t15076: F) -> (F, F, F, F, F) {
    let t15078 = t5172 * t3452;
    let t15080 = t10787 * t5062;
    let t15082 = t5047 * t14097;
    let t15083 = t5046 * t15082;
    let t15085 = t251 * t10506;
    let t15086 = t15085 * t14611;
    let t15087 = t5180 * t15086;
    let t15089 = -t15061 / F::cast_from(192.0_f64) + t15063 / F::cast_from(18.0_f64) - t15066 / F::cast_from(64.0_f64) - t15069 / F::cast_from(12.0_f64) - F::cast_from(11.0_f64) / F::cast_from(18.0_f64) * t15072 - t15074 / F::cast_from(24.0_f64) - t15076 / F::cast_from(24.0_f64) + t15078 / F::cast_from(256.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t15080 + t15083 / F::cast_from(8.0_f64) + t15087 / F::cast_from(864.0_f64);
    (t15078, t15080, t15083, t15087, t15089)
}
