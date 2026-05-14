//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 920/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk920<F: Float>(t2225: F, t6518: F, t1541: F, t525: F, t524: F, t5054: F, t506: F, t529: F, t146: F, t5052: F) -> (F, F, F, F, F) {
    let t6519 = t6518 * t2225;
    let t6521 = t525 * t1541;
    let t6522 = t524 * t6521;
    let t6523 = t506 * t5054;
    let t6524 = t529 * t6523;
    let t6527 = t146 * t5052;
    (t6519, t6522, t6523, t6524, t6527)
}
