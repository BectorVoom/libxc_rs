//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1220/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1220<F: Float>(t261: F, t3304: F, t9311: F, t11741: F, t11748: F, t146: F, t2206: F, t3177: F, t3305: F, t2124: F, t30049: F, t3295: F) -> (F, F, F, F) {
    let t43559 = t3304 * t261 * t9311;
    let t43561 = t11748 * t11741;
    let t43564 = t146 * t2206 * t3177;
    let t43565 = t43564 * t3305;
    let t43569 = t3295 * t2124 * t30049;
    (t43559, t43561, t43565, t43569)
}
