//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 805/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk805<F: Float>(t3195: F, t6475: F, t2380: F, t1167: F, t179: F, t6380: F, t404: F, t2099: F, t3237: F, t3235: F, t178: F, t6457: F) -> (F, F, F, F, F, F, F) {
    let t8392 = t6475 * t3195;
    let t8394 = F::new(0.57165357490759649296e-3) * t2380 * t8392;
    let t8397 = t179 * t6380 * t1167;
    let t8398 = t404 * t8397;
    let t8406 = t2099 * t3237;
    let t8408 = F::new(0.17149607247227894789e-2) * t3235 * t8406;
    let t8427 = t6457 * t178;
    (t8392, t8394, t8397, t8398, t8406, t8408, t8427)
}
