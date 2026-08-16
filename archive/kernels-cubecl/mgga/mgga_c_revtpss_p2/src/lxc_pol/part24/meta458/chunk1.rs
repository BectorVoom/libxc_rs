//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1429/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1429<F: Float>(t16170: F, t372: F, t11773: F, t15925: F, t1041: F, t1670: F, t42994: F, t12046: F, t1647: F, t4746: F, t4995: F, t15669: F, t3286: F) -> (F, F, F, F, F, F) {
    let t55122 = t372 * t16170;
    let t55141 = t15925 * t11773;
    let t55247 = t1041 * t42994 * t1670;
    let t55599 = t1647 * t12046;
    let t55732 = t4746 * t4995;
    let t55747 = t15669 * t3286;
    (t55122, t55141, t55247, t55599, t55732, t55747)
}
