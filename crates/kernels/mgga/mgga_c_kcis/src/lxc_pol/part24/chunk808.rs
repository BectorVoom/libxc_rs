//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 808/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk808<F: Float>(t11: F, t41: F, t85: F, t5143: F, t5135: F, t1788: F, t752: F, t1791: F, t318: F, t86: F, t119: F, t339: F, t9368: F) -> (F, F, F, F, F, F) {
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t14956 = t14955 * t5143;
    let t14959 = F::new(0.5895802469135802469e-1) * t14955 * t5135;
    let t14966 = t752 * t1788;
    let t14996 = t86 * t318 * t1791;
    let t15007 = t119 * t41;
    let t15008 = t85 * t15007;
    let t15022 = t9368 * t339;
    (t14956, t14959, t14966, t14996, t15008, t15022)
}
