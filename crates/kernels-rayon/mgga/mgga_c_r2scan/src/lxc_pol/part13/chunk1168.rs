//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1168/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1168(t22868: f64, t27182: f64, t3332: f64, t10868: f64, t6165: f64, t8156: f64, t22790: f64, t25813: f64, t8160: f64, t26185: f64, t26186: f64, t1054: f64, t5108: f64, t7963: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39992 = t22868 * t3332 * t27182;
    let t39995 = t6165 * t10868 * t8156;
    let t39996 = 0.13972381860938637374e0_f64 * t39995;
    let t39998 = t22790 * t3332 * t25813;
    let t40001 = t6165 * t10868 * t8160;
    let t40004 = t26185 * t3332 * t26186;
    let t40007 = t5108 * t1054 * t7963;
    (t39992, t39996, t39998, t40001, t40004, t40007)
}
