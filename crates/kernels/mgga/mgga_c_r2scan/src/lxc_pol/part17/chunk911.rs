//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 911/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk911<F: Float>(t12365: F, t374: F, t1039: F, t3570: F, t1149: F, t2449: F, t11554: F, t986: F, t11496: F, t3574: F, t983: F, t2892: F, t797: F, t106: F, t3055: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t12366 = t12365 * t374;
    let t12367 = t1039 * t3570;
    let t12368 = t2449 * t1149;
    let t12383 = t11554 * t986;
    let t12391 = t11496 * t986;
    let t12395 = t3574 * t983;
    let t12414 = t797 * t2892;
    let t12422 = t97 * t106 * t3055;
    (t12366, t12367, t12368, t12383, t12391, t12395, t12414, t12422)
}
