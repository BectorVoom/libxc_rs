//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 864/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk864<F: Float>(t8701: F, t950: F, t628: F, t8165: F, t4641: F, t4913: F, t8697: F, t8699: F, t8702: F, t8704: F, t8710: F, t8712: F) -> (F, F, F) {
    let t8714 = t950 * t8701;
    let t8716 = t628 * t8165;
    let t8719 = -F::cast_from(2.8769444444444443_f64) * t8697 + F::cast_from(27.618666666666666_f64) * t8699 - F::cast_from(10.229135802469136_f64) * t8702 + F::cast_from(8.950493827160495_f64) * t8704 + F::cast_from(3.131074074074074_f64) * t4641 + F::new(0.0366775) * t8710 - F::new(0.58684) * t8712 + F::cast_from(0.6520444444444444_f64) * t8714 + F::cast_from(0.5705388888888889_f64) * t8716 + F::cast_from(1.3490888888888888_f64) * t4913;
    (t8714, t8716, t8719)
}
