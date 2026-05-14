//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 699/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk699<F: Float>(t1427: F, t2572: F, t996: F, t3596: F, t986: F, t995: F, t2594: F) -> (F, F, F, F) {
    let t3613 = t2572 * t1427;
    let t3614 = t3613 * t996;
    let t3618 = t986 * t3596 * t995;
    let t3621 = t2594 * t1427;
    (t3613, t3614, t3618, t3621)
}
