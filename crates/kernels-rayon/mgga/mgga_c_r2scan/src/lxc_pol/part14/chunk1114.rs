//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1114/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1114(t10772: f64, t3308: f64, t7978: f64, t8006: f64, t2547: f64, t37764: f64, t10781: f64, t8039: f64, t3295: f64, t8014: f64, t7974: f64, t25397: f64, t37945: f64, t38031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39413 = t10772 * t3308 * t7978;
    let t39416 = t10772 * t3308 * t8006;
    let t39420 = t37764 * t2547;
    let t39422 = t10781 * t8039;
    let t39424 = t3295 * t8014;
    let t39426 = t3295 * t7974;
    let t39429 = t38031 * t37945 * t25397;
    (t39413, t39416, t39420, t39422, t39424, t39426, t39429)
}
