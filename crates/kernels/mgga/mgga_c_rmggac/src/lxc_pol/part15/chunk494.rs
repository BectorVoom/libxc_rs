//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 494/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk494<F: Float>(t623: F, t837: F, t234: F, t321: F, t1598: F, t1652: F, t1953: F, t68: F, t131: F, t1926: F, t333: F, t1933: F, t1734: F, t338: F, t352: F, t551: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6473 = t623 * t837;
    let t6477 = t234 * t321;
    let t6482 = t1598 * t1652;
    let t6491 = t68 * t1953;
    let t6492 = t6491 * t131;
    let t6495 = t1926 * t333;
    let t6501 = t1933 * t321;
    let t6504 = t1933 * t333;
    let t6507 = t338 * t1734;
    let t6508 = t6507 * t352;
    let t6522 = t551 * t570;
    (t6473, t6477, t6482, t6491, t6492, t6495, t6501, t6504, t6508, t6522)
}
