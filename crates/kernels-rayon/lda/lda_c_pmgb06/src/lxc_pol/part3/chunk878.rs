//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 878/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk878(t1050: f64, t1055: f64, t1062: f64, t109: f64, t138: f64, t273: f64, t3709: f64, t3741: f64, t3797: f64, t3808: f64, t409: f64, t4641: f64, t4913: f64, t661: f64, t667: f64, t8599: f64, t8626: f64, t8629: f64, t8633: f64, t8637: f64, t8640: f64, t8644: f64, t8647: f64, t8651: f64, t8655: f64, t8688: f64, t8697: f64, t8699: f64, t8702: f64, t8704: f64, t8710: f64, t8712: f64, t8714: f64, t8716: f64, t8733: f64, t963: f64, t978: f64, t991: f64, t994: f64) -> f64 {
    let t8925 = t8626 + 1.2842595503380418_f64 * t138 * t409 * t963 * t1062 - 0.02168716260060348_f64 * t138 * t1050 * t3797 + t8629 + t8633 + t8637 - t8640 - t8644 + t8647 - 0.27397333333333335_f64 * t138 * t409 * t991 * t994 - 0.08674865040241392_f64 * t138 * t409 * t978 * t1055 + 3.8527786510141255_f64 * t138 * t109 * t3709 * t3808 + t8651 - t8655 - t8733 + 1.0_f64 * t661 * (-3.9219166666666667_f64 * t8697 + 37.6504_f64 * t8699 - 13.944592592592592_f64 * t8702 + 12.201518518518519_f64 * t8704 + 5.356037037037037_f64 * t4641 + 0.14025833333333335_f64 * t8710 - 2.2441333333333335_f64 * t8712 + 2.4934814814814814_f64 * t8714 + 2.1817962962962962_f64 * t8716 + 1.6979925925925925_f64 * t4913) * t667 - 12304.822629859687_f64 * t273 * t8688 * t8599 * t3741;
    t8925
}
