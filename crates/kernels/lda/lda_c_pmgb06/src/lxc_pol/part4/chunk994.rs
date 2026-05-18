//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 994/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk994<F: Float>(t628: F, t8165: F, t4641: F, t4913: F, t8697: F, t8699: F, t8702: F, t8704: F, t8710: F, t8712: F, t8714: F, t675: F, t682: F, t696: F) -> (F, F, F) {
    let t8716 = t628 * t8165;
    let t8719 = -F::new(2.8769444444444443) * t8697 + F::new(27.618666666666666) * t8699 - F::new(10.229135802469136) * t8702 + F::new(8.950493827160495) * t8704 + F::new(3.131074074074074) * t4641 + F::new(0.0366775) * t8710 - F::new(0.58684) * t8712 + F::new(0.6520444444444444) * t8714 + F::new(0.5705388888888889) * t8716 + F::new(1.3490888888888888) * t4913;
    let t8723 = F::new(0.5848223622634646) * t696 * t675 * t8719 * t682;
    (t8716, t8719, t8723)
}
