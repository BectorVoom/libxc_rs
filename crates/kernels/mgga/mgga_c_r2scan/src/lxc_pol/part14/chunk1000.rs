//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1000/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1000<F: Float>(t10772: F, t3308: F, t7978: F, t8006: F, t2547: F, t37764: F, t10781: F, t8039: F, t3295: F, t8014: F, t7974: F, t25397: F, t37945: F, t38031: F, t8018: F, t1577: F, t7438: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39413 = t10772 * t3308 * t7978;
    let t39416 = t10772 * t3308 * t8006;
    let t39420 = t37764 * t2547;
    let t39422 = t10781 * t8039;
    let t39424 = t3295 * t8014;
    let t39426 = t3295 * t7974;
    let t39429 = t38031 * t37945 * t25397;
    let t39431 = t3295 * t8018;
    let t39434 = t1577 * t3308 * t7438;
    (t39413, t39416, t39420, t39422, t39424, t39426, t39429, t39431, t39434)
}
