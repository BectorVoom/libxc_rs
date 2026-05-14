//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1045/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1045<F: Float>(t11741: F, t11748: F, t146: F, t2206: F, t3177: F, t3305: F, t2124: F, t30049: F, t3295: F, t30053: F, t3308: F, t5136: F, t30057: F, t6218: F, t11711: F, t8240: F) -> (F, F, F, F, F, F) {
    let t43561 = t11748 * t11741;
    let t43564 = t146 * t2206 * t3177;
    let t43565 = t43564 * t3305;
    let t43569 = t3295 * t2124 * t30049;
    let t43572 = t5136 * t3308 * t30053;
    let t43575 = t6218 * t3308 * t30057;
    let t43577 = t8240 * t11711;
    (t43561, t43565, t43569, t43572, t43575, t43577)
}
