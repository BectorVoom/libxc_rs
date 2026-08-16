//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 862/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk862<F: Float>(t962: F, t966: F, t696: F, t8599: F, t3742: F, t971: F, t977: F, t3741: F, t3760: F, t138: F, t28: F, t4238: F, t8333: F) -> (F, F, F, F, F, F, F, F) {
    let t8677 = t962 * t962;
    let t8678 = F::cast_from(1.0_f64) / t8677;
    let t8680 = t966 * t966;
    let t8681 = F::cast_from(1.0_f64) / t8680;
    let t8684 = F::cast_from(91082.60419215256_f64) * t696 * t8678 * t8599 * t8681;
    let t8685 = t971 * t3742;
    let t8688 = F::cast_from(1.0_f64) / t962 / t977;
    let t8692 = F::cast_from(12304.822629859687_f64) * t696 * t8688 * t8599 * t3741;
    let t8693 = t971 * t3760;
    let t8697 = t8333 * t28 * t4238 * t138;
    (t8678, t8681, t8684, t8685, t8688, t8692, t8693, t8697)
}
