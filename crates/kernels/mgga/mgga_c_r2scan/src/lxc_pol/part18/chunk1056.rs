//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1056/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1056<F: Float>(t11696: F, t40075: F, t10710: F, t10728: F, t27955: F, t11699: F, t39961: F, t3281: F, t9236: F, t3606: F, t39840: F, t7624: F, t2184: F, t30213: F, t3308: F, t12547: F, t6425: F) -> (F, F, F, F, F, F, F) {
    let t43281 = t40075 * t11696;
    let t43284 = t10728 * t10710 * t27955;
    let t43286 = t39961 * t11699;
    let t43288 = t3281 * t9236;
    let t43291 = t39840 * t3606 * t7624;
    let t43294 = t2184 * t3308 * t30213;
    let t43296 = t6425 * t12547;
    (t43281, t43284, t43286, t43288, t43291, t43294, t43296)
}
