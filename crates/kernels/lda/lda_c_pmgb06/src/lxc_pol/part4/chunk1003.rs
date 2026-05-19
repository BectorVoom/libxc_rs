//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1003/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1003<F: Float>(t1050: F, t1055: F, t1062: F, t109: F, t138: F, t273: F, t3709: F, t3741: F, t3797: F, t3808: F, t409: F, t4641: F, t4913: F, t661: F, t667: F, t8599: F, t8626: F, t8629: F, t8633: F, t8637: F, t8640: F, t8644: F, t8647: F, t8651: F, t8655: F, t8688: F, t8697: F, t8699: F, t8702: F, t8704: F, t8710: F, t8712: F, t8714: F, t8716: F, t8733: F, t963: F, t978: F, t991: F, t994: F) -> F {
    let t8925 = t8626 + F::cast_from(1.2842595503380418_f64) * t138 * t409 * t963 * t1062 - F::cast_from(0.02168716260060348_f64) * t138 * t1050 * t3797 + t8629 + t8633 + t8637 - t8640 - t8644 + t8647 - F::cast_from(0.27397333333333335_f64) * t138 * t409 * t991 * t994 - F::cast_from(0.08674865040241392_f64) * t138 * t409 * t978 * t1055 + F::cast_from(3.8527786510141255_f64) * t138 * t109 * t3709 * t3808 + t8651 - t8655 - t8733 + F::new(1.0) * t661 * (-F::cast_from(3.9219166666666667_f64) * t8697 + F::new(37.6504) * t8699 - F::cast_from(13.944592592592592_f64) * t8702 + F::cast_from(12.201518518518519_f64) * t8704 + F::cast_from(5.356037037037037_f64) * t4641 + F::cast_from(0.14025833333333335_f64) * t8710 - F::cast_from(2.2441333333333335_f64) * t8712 + F::cast_from(2.4934814814814814_f64) * t8714 + F::cast_from(2.1817962962962962_f64) * t8716 + F::cast_from(1.6979925925925925_f64) * t4913) * t667 - F::cast_from(12304.822629859687_f64) * t273 * t8688 * t8599 * t3741;
    t8925
}
