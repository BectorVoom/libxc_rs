//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 861/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk861<F: Float>(t1037: F, t1064: F, t3166: F, t358: F, t3017: F, t344: F, t935: F, t334: F, t913: F, t904: F, t907: F, t319: F, t4606: F, t5021: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F, t8157: F, t8159: F, t8161: F) -> (F, F, F, F, F, F) {
    let t8208 = t1064 * t1037;
    let t8212 = t3166 * t358;
    let t8216 = t344 * t3017;
    let t8218 = t935 * t935;
    let t8221 = F::cast_from(6.0_f64) * t913 * t8218 * t334;
    let t8224 = F::cast_from(48.24547296645331_f64) * t904 * t8218 * t907;
    let t8238 = F::cast_from(1.0_f64) * t319 * (-F::cast_from(2.109916666666667_f64) * t8141 + F::cast_from(20.2552_f64) * t8143 - F::cast_from(7.501925925925926_f64) * t8146 + F::cast_from(6.564185185185186_f64) * t8149 + F::cast_from(3.100395061728395_f64) * t4606 + F::cast_from(0.06825833333333334_f64) * t8155 - F::cast_from(1.0921333333333334_f64) * t8157 + F::cast_from(1.2134814814814814_f64) * t8159 + F::cast_from(1.0617962962962963_f64) * t8161 + F::cast_from(1.3388493827160495_f64) * t5021) * t334;
    (t8208, t8212, t8216, t8221, t8224, t8238)
}
