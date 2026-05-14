//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 962/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk962<F: Float>(t1227: F, t5728: F, t2368: F, t6517: F, t406: F, t6524: F, t8427: F) -> (F, F, F, F, F) {
    let t8429 = t1227 * t5728;
    let t8430 = t6517 * t2368;
    let t8431 = t8429 * t8430;
    let t8432 = t406 * t8431;
    let t8435 = t6524 * t8427;
    (t8429, t8430, t8431, t8432, t8435)
}
