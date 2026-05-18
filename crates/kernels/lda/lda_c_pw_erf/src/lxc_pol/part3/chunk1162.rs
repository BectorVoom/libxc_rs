//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1162/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1162<F: Float>(t1351: F, t588: F, t1370: F, t3604: F, t3586: F, t3589: F, t213: F, t11866: F, t11909: F, t10092: F, t10098: F, t10100: F, t10115: F, t10204: F, t10206: F, t10208: F, t10210: F, t10212: F, t10225: F, t10250: F, t10252: F, t11854: F, t13294: F, t13344: F, t1371: F, t2061: F, t3609: F, t3618: F, t589: F) -> (F, F) {
    let t13631 = t588 * t1351;
    let t13635 = t1370 * t3604;
    let t13639 = t3586 * t3589;
    let t13643 = t213 * t1351;
    let t13645 = t11866 * t13643 * t11909;
    let t13647 = F::new(0.044444444444444446) * t10204 - F::new(0.022222222222222223) * t10206 - F::new(0.007407407407407408) * t10208 + F::new(0.0044444444444444444) * t10210 + F::new(0.0019753086419753087) * t10212 + t10225 + F::new(0.09597777777777777) * t10092 - F::new(0.03199259259259259) * t10098 + F::new(0.013330246913580247) * t10100 - F::new(0.047988888888888886) * t10115 - F::new(0.02666666666666667) * t10250 + F::new(0.0044444444444444444) * t10252 - F::new(0.08) * t2061 * t589 * t3618 + F::new(0.013333333333333334) * t2061 * t1371 * t3609 - F::new(0.08) * t2061 * t1371 * t13344 + F::new(0.24) * t2061 * t589 * t13294 - F::new(0.12) * t11854 * t13631 * t11909 + F::new(0.04) * t11854 * t13635 * t11909 - F::new(0.008888888888888889) * t11854 * t13639 * t11909 - F::new(0.64785) * t13645;
    (t13645, t13647)
}
