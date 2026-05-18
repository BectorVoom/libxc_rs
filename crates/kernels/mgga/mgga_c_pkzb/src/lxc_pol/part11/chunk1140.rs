//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1140/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1140<F: Float>(t237: F, t9462: F, t1976: F, t9203: F, t1878: F, t218: F, t3542: F, t675: F, t9187: F, t9194: F, t9198: F, t3546: F) -> (F, F, F, F, F, F, F) {
    let t25656 = t237 * t9462;
    let t25671 = t1976 * t9203;
    let t25734 = t218 * t1878 * t3542;
    let t25740 = t218 * t675 * t9187;
    let t25747 = t218 * t675 * t9194;
    let t25750 = t218 * t675 * t9198;
    let t25767 = t218 * t1878 * t3546;
    (t25656, t25671, t25734, t25740, t25747, t25750, t25767)
}
