//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 963/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk963<F: Float>(t1464: F, t1639: F, t5071: F, t5138: F, t2865: F, t3032: F, t5077: F, t822: F, t2965: F, t5078: F, t1601: F, t12693: F, t2918: F, t518: F, t12531: F, t2952: F) -> (F, F, F, F, F, F) {
    let t13053 = t1639 * t1464;
    let t13056 = 2.0 / 9.0 * t5138 * t13053 * t5071;
    let t13060 = 2.0 / 5.0 * t5077 * t3032 * t822 * t2865;
    let t13063 = 4.0 / 15.0 * t5077 * t5078 * t2965;
    let t13064 = t1601 * t1464;
    let t13067 = 2.0 / 9.0 * t5138 * t13064 * t12693;
    let t13068 = t518 * t2918;
    let t13071 = 2.0 / 3.0 * t5138 * t13068 * t12531;
    let t13074 = 2.0 / 15.0 * t5077 * t5078 * t2952;
    (t13056, t13060, t13063, t13067, t13071, t13074)
}
