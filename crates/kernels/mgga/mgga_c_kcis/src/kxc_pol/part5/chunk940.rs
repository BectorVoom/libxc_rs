//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 940/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk940<F: Float>(t119: F, t41: F, t85: F, t339: F, t9368: F, t238: F, t5158: F, t86: F, t284: F, t5082: F, t10506: F, t251: F, t1281: F, t5358: F, t13101: F, t13103: F) -> (F, F, F, F, F, F, F, F) {
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15022 = t9368 * t339;
    let t15046 = 0.53062222222222222222e-1 * t86 * t238 * t5158;
    let t15068 = t5082 * t284;
    let t15085 = t251 * t10506;
    let t15109 = t5358 * t1281;
    let t15112 = 0.23214722222222222222e-2 * t13101;
    let t15113 = 0.15476481481481481481e-2 * t13103;
    (t15008, t15022, t15046, t15068, t15085, t15109, t15112, t15113)
}
