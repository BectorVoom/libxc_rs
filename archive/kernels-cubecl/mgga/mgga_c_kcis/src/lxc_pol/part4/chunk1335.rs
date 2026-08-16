//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1335/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1335<F: Float>(t1971: F, t2471: F, t1976: F, t2475: F, t1968: F, t2466: F, t13589: F, t5839: F, t13577: F, t5842: F, t13583: F, t5836: F) -> (F, F, F, F, F, F) {
    let t17197 = t2471 * t1971;
    let t17199 = t2475 * t1976;
    let t17201 = t2466 * t1968;
    let t17203 = t13589 * t5839;
    let t17205 = t13577 * t5842;
    let t17207 = t13583 * t5836;
    (t17197, t17199, t17201, t17203, t17205, t17207)
}
