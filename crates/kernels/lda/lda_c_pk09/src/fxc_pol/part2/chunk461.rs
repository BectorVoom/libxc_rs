//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 461/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk461<F: Float>(t1325: F, t1327: F, t2502: F, t2505: F, t337: F, t281: F, t130: F, t2143: F, t333: F) -> (F, F, F, F, F) {
    let t2507 = t1325 - F::new(2.0) * t2502 + t1327 + F::new(2.0) * t2505;
    let t2508 = t2507 * t337;
    let t2509 = t2508 * t281;
    let t2512 = t130 * t2143;
    let t2513 = t333 * t2512;
    (t2507, t2508, t2509, t2512, t2513)
}
