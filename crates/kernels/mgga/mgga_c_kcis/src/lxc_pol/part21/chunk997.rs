//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 997/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk997<F: Float>(t3452: F, t5172: F, t10787: F, t5062: F, t14097: F, t5047: F, t5046: F, t10506: F, t251: F, t14611: F, t5180: F, t15061: F, t15063: F, t15066: F, t15069: F, t15072: F, t15074: F, t15076: F) -> (F, F, F, F, F, F, F) {
    let t15078 = t5172 * t3452;
    let t15080 = t10787 * t5062;
    let t15082 = t5047 * t14097;
    let t15083 = t5046 * t15082;
    let t15085 = t251 * t10506;
    let t15086 = t15085 * t14611;
    let t15087 = t5180 * t15086;
    let t15089 = -t15061 / F::new(192.0) + t15063 / F::new(18.0) - t15066 / F::new(64.0) - t15069 / F::new(12.0) - F::new(11.0) / F::new(18.0) * t15072 - t15074 / F::new(24.0) - t15076 / F::new(24.0) + t15078 / F::new(256.0) + F::new(2.0) / F::new(9.0) * t15080 + t15083 / F::new(8.0) + t15087 / F::new(864.0);
    (t15078, t15080, t15082, t15083, t15086, t15087, t15089)
}
