//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1171/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1171<F: Float>(t122: F, t3434: F, t3437: F, t40453: F, t1563: F, t2867: F, t10831: F, t1102: F, t3692: F, t1543: F, t3582: F, t2333: F, t2526: F) -> (F, F, F, F, F) {
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40464 = t2867 * t1563;
    let t40485 = t1102 * t10831 * t3692;
    let t40487 = t3582 * t1543;
    let t40491 = t2333 * t2526;
    (t40460, t40464, t40485, t40487, t40491)
}
