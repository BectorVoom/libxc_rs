//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1172/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1172(t3281: f64, t9236: f64, t3606: f64, t39840: f64, t7624: f64, t2184: f64, t30213: f64, t3308: f64, t12547: f64, t6425: f64, t1592: f64, t27996: f64) -> (f64, f64, f64, f64, f64) {
    let t43288 = t3281 * t9236;
    let t43291 = t39840 * t3606 * t7624;
    let t43294 = t2184 * t3308 * t30213;
    let t43296 = t6425 * t12547;
    let t43299 = t1592 * t3308 * t27996;
    (t43288, t43291, t43294, t43296, t43299)
}
