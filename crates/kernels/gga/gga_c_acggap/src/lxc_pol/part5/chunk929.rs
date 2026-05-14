//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 929/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk929<F: Float>(t1089: F, t175: F, t3101: F, t384: F, t495: F, t1429: F, t3670: F, t14423: F, t172: F, t12610: F, t1432: F, t398: F, t13298: F, t13299: F, t3196: F, t1165: F, t1532: F, t15758: F, t3451: F) -> (F, F, F, F, F, F) {
    let t17409 = t384 * t1089 * t175 * t495 * t3101;
    let t17411 = t3670 * t1429;
    let t17413 = t172 * t14423;
    let t17421 = t384 * t398 * t12610 * t1432;
    let t17430 = t13298 * t13299 * t1432 * t3196;
    let t17436 = t3451 * t1165 * t1532 * t15758;
    (t17409, t17411, t17413, t17421, t17430, t17436)
}
