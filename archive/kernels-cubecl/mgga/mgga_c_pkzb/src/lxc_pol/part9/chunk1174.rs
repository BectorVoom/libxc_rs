//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1174/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1174<F: Float>(t1727: F, t7087: F, t2663: F, t5296: F, t1025: F, t16378: F, t5250: F, t6859: F, t5239: F, t6897: F, t16399: F, t6946: F) -> (F, F, F, F, F, F) {
    let t20405 = t1727 * t7087;
    let t20407 = t5296 * t2663;
    let t20409 = t16378 * t1025;
    let t20419 = t6859 * t5250;
    let t20427 = t6897 * t5239;
    let t20436 = t16399 * t6946;
    (t20405, t20407, t20409, t20419, t20427, t20436)
}
