//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 871/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk871<F: Float>(t312: F, t8076: F, t2696: F, t2699: F, t2702: F, t2708: F, t2711: F, t2738: F, t2747: F, t2751: F, t2754: F, t2758: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8077 = t8076 * t312;
    let t8097 = F::cast_from(1.8960024086108225_f64) * t2696;
    let t8098 = F::cast_from(0.06506148529668915_f64) * t2699;
    let t8099 = F::cast_from(1.9263778438055648_f64) * t2702;
    let t8101 = F::cast_from(0.1301229705933783_f64) * t2708;
    let t8102 = F::cast_from(0.08674864706225219_f64) * t2711;
    let t8103 = F::cast_from(2.339289358982082_f64) * t2738;
    let t8106 = F::cast_from(3.436685857643691_f64) * t2747;
    let t8107 = F::cast_from(0.2849333333333333_f64) * t2751;
    let t8108 = F::cast_from(0.2137_f64) * t2754;
    let t8109 = F::cast_from(0.4274_f64) * t2758;
    (t8077, t8097, t8098, t8099, t8101, t8102, t8103, t8106, t8107, t8108, t8109)
}
