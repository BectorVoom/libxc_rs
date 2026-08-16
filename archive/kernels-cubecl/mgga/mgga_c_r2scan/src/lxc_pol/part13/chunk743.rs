//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 743/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk743<F: Float>(t6028: F, t6029: F, t1375: F, t2060: F, t2062: F, t1783: F, t759: F, t761: F, t2049: F, t607: F, t4733: F, t4736: F, t4739: F, t5860: F) -> (F, F, F, F, F) {
    let t6030 = t6028 * t6029;
    let t6032 = t2060 * t1375;
    let t6033 = t6032 * t2062;
    let t6036 = t759 * t1783 * t761;
    let t6038 = t607 * t2049;
    let t6039 = t759 * t6038;
    let t6044 = -F::cast_from(0.29633333333333333333e-1_f64) * t4733 + F::cast_from(0.19755555555555555555e-1_f64) * t4736 - F::cast_from(0.23048148148148148148e-1_f64) * t4739 - t5860;
    (t6030, t6033, t6036, t6039, t6044)
}
