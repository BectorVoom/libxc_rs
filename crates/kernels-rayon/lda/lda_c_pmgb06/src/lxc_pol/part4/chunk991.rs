//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 991/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk991(t1112: f64, t3720: f64, t1062: f64, t3709: f64, t696: f64, t957: f64, t3745: f64, t980: f64, t962: f64, t966: f64, t8599: f64, t3742: f64, t971: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8663 = t3720 * t1112;
    let t8668 = 623.3709278045327_f64 * t696 * t3709 * t957 * t1062;
    let t8675 = t3745 * t980;
    let t8677 = t962 * t962;
    let t8678 = 1.0_f64 / t8677;
    let t8680 = t966 * t966;
    let t8681 = 1.0_f64 / t8680;
    let t8684 = 91082.60419215256_f64 * t696 * t8678 * t8599 * t8681;
    let t8685 = t971 * t3742;
    (t8663, t8668, t8675, t8678, t8681, t8684, t8685)
}
