//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1003/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1003<F: Float>(t3678: F, t6755: F, t12005: F, t1348: F, t6767: F, t11561: F, t11863: F, t11864: F, t11618: F, t11623: F, t11631: F, t11634: F, t11637: F, t12020: F, t11858: F, t39464: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41039 = t6755 * t3678;
    let t41042 = t1348 * t12005;
    let t41047 = t6767 * t3678;
    let t41104 = 5.0 / 8.0 * t11561;
    let t41105 = 2.0 * t11863;
    let t41106 = 2.0 * t11864;
    let t41107 = 5.0 / 8.0 * t11618;
    let t41108 = 45.0 / 32.0 * t11623;
    let t41109 = 5.0 / 8.0 * t11631;
    let t41110 = t11634 / 2.0;
    let t41111 = 3.0 / 2.0 * t11637;
    let t41112 = 2.0 * t12020;
    let t41113 = t11858 / 2.0;
    let t41395 = 0.11902492299418487743e0 * t39464;
    (t41039, t41042, t41047, t41104, t41105, t41106, t41107, t41108, t41109, t41110, t41111, t41112, t41113, t41395)
}
