//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 690/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk690<F: Float>(t166: F, t6006: F, t6007: F, t2055: F, t2056: F, t607: F, t2050: F, t761: F, t2054: F, t58: F, t423: F, t597: F, t2049: F, t759: F, t4733: F, t4736: F, t4739: F, t5860: F) -> (F, F, F, F, F, F, F, F) {
    let t6010 = 0.1714584e0 * t6006 * t166 * t6007;
    let t6012 = t2055 * t607 * t2056;
    let t6026 = 0.1714584e0 * t2055 * t2050 * t761;
    let t6027 = t2054 * t58;
    let t6028 = t6027 * t423;
    let t6029 = t597 * t2056;
    let t6030 = t6028 * t6029;
    let t6038 = t607 * t2049;
    let t6039 = t759 * t6038;
    let t6044 = -0.29633333333333333333e-1 * t4733 + 0.19755555555555555555e-1 * t4736 - 0.23048148148148148148e-1 * t4739 - t5860;
    (t6010, t6012, t6026, t6027, t6029, t6030, t6039, t6044)
}
