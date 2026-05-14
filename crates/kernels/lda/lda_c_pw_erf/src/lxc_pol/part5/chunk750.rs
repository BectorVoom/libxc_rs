//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 750/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk750<F: Float>(t1460: F, t7354: F, t522: F, t519: F, t1486: F, t7365: F, t574: F, t571: F, t2171: F, t2566: F, t3722: F, t1459: F, t3714: F, t1485: F, t2146: F, t2562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7474 = t1460 * t7354;
    let t7475 = t522 * t7474;
    let t7477 = 8.0 / 15.0 * t519 * t7475;
    let t7478 = t1486 * t7365;
    let t7479 = t574 * t7478;
    let t7481 = 8.0 / 15.0 * t571 * t7479;
    let t7483 = 8.0 / 15.0 * t2171 * t2566;
    let t7484 = t3722 * t7354;
    let t7485 = t1459 * t7484;
    let t7487 = 8.0 / 9.0 * t519 * t7485;
    let t7488 = t3714 * t7365;
    let t7489 = t1485 * t7488;
    let t7491 = 8.0 / 9.0 * t571 * t7489;
    let t7493 = 8.0 / 15.0 * t2146 * t2562;
    (t7474, t7475, t7477, t7478, t7479, t7481, t7483, t7484, t7485, t7487, t7488, t7489, t7491, t7493)
}
