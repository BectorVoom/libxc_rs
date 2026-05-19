//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 991/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk991<F: Float>(t1112: F, t3720: F, t1062: F, t3709: F, t696: F, t957: F, t3745: F, t980: F, t962: F, t966: F, t8599: F, t3742: F, t971: F) -> (F, F, F, F, F, F, F) {
    let t8663 = t3720 * t1112;
    let t8668 = F::cast_from(623.3709278045327_f64) * t696 * t3709 * t957 * t1062;
    let t8675 = t3745 * t980;
    let t8677 = t962 * t962;
    let t8678 = F::new(1.0) / t8677;
    let t8680 = t966 * t966;
    let t8681 = F::new(1.0) / t8680;
    let t8684 = F::cast_from(91082.60419215256_f64) * t696 * t8678 * t8599 * t8681;
    let t8685 = t971 * t3742;
    (t8663, t8668, t8675, t8678, t8681, t8684, t8685)
}
