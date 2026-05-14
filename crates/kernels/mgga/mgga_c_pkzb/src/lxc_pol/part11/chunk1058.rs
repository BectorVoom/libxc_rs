//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1058/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1058<F: Float>(t10414: F, t16111: F, t440: F, t1429: F, t3314: F, t8: F, t3318: F, t973: F, t1430: F, t2489: F, t8635: F, t10422: F, t1424: F, t16129: F, t82: F, t15: F) -> (F, F, F, F, F, F, F, F) {
    let t28649 = t16111 * t10414 * t440;
    let t28653 = t3314 * t8 * t1429;
    let t28658 = t973 * t3318;
    let t28659 = t28658 * t440;
    let t28662 = t1430 * t3318;
    let t28665 = t2489 * t8635;
    let t28671 = t1424 * t10422 * t440;
    let t28676 = 6.0 * t82 + 12.0 * t16129;
    let t28677 = t15 * t28676;
    (t28649, t28653, t28659, t28662, t28665, t28671, t28676, t28677)
}
