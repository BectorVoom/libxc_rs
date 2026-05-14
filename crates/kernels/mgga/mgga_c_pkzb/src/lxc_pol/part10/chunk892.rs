//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 892/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk892<F: Float>(t326: F, t6514: F, t2370: F, t394: F, t5728: F, t2362: F, t5717: F) -> (F, F, F, F) {
    let t6515 = t6514 * t326;
    let t6517 = t2370 * t394;
    let t6518 = t5728 * t6517;
    let t6523 = t5717 * t2362;
    (t6515, t6517, t6518, t6523)
}
