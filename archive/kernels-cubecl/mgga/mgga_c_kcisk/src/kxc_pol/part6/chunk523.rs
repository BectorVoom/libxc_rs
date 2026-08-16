//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 523/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk523<F: Float>(t1308: F, t6458: F, t2321: F, t3973: F, t1580: F, t2327: F, t4419: F, t535: F, t2326: F, t4374: F, t1528: F, t2285: F) -> (F, F, F, F, F, F, F) {
    let t6459 = t6458 * t1308;
    let t6473 = t3973 * t2321;
    let t6474 = t1580 * t6473;
    let t6497 = t4419 * t2327;
    let t6498 = t535 * t6497;
    let t6505 = t4374 * t2326;
    let t6518 = t2285 * t1528;
    (t6459, t6473, t6474, t6497, t6498, t6505, t6518)
}
