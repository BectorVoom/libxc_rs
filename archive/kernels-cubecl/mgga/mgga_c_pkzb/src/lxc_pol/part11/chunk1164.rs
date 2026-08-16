//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1164/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1164<F: Float>(t3880: F, t937: F, t2393: F, t10365: F, t2464: F, t10414: F, t16111: F, t440: F, t1429: F, t3314: F, t8: F, t3318: F, t973: F) -> (F, F, F, F, F) {
    let t28492 = t937 * t3880;
    let t28493 = t2393 * t28492;
    let t28595 = t10365 * t2464;
    let t28649 = t16111 * t10414 * t440;
    let t28653 = t3314 * t8 * t1429;
    let t28658 = t973 * t3318;
    (t28493, t28595, t28649, t28653, t28658)
}
