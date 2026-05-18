//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 604/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk604<F: Float>(t1405: F, t565: F, t1284: F, t1397: F, t1404: F, t514: F, t211: F, t1508: F, t544: F, t1302: F, t2114: F, t1513: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3442 = F::new(4.0) / F::new(5.0) * t565 * t1405;
    let t3443 = t1284 * t1397;
    let t3444 = F::new(16.0) / F::new(15.0) * t3443;
    let t3445 = t514 * t1404;
    let t3446 = t211 * t3445;
    let t3447 = F::new(8.0) / F::new(15.0) * t3446;
    let t3449 = F::new(2.0) / F::new(5.0) * t1508 * t544;
    let t3451 = F::new(4.0) / F::new(5.0) * t2114 * t1302;
    let t3453 = F::new(4.0) / F::new(5.0) * t1513 * t544;
    (t3442, t3443, t3444, t3445, t3446, t3447, t3449, t3451, t3453)
}
