//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 848/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk848<F: Float>(t1971: F, t2471: F, t1976: F, t2475: F, t1968: F, t2466: F, t13589: F, t5839: F, t13577: F, t5842: F, t13583: F, t5836: F, t1437: F, t16073: F, t1430: F, t1451: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17197 = t2471 * t1971;
    let t17199 = t2475 * t1976;
    let t17201 = t2466 * t1968;
    let t17203 = t13589 * t5839;
    let t17205 = t13577 * t5842;
    let t17207 = t13583 * t5836;
    let t17210 = t1437 * t16073;
    let t17213 = t1430 * t16073;
    let t17216 = t1451 * t16073;
    (t17197, t17199, t17201, t17203, t17205, t17207, t17210, t17213, t17216)
}
