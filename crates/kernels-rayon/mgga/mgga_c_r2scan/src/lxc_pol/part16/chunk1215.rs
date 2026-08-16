//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1215/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1215(t12455: f64, t3336: f64, t5103: f64, t11659: f64, t7601: f64, t2184: f64, t30281: f64, t3308: f64, t10810: f64, t1577: f64, t9296: f64, t3602: f64, t40062: f64, t8089: f64) -> (f64, f64, f64, f64, f64) {
    let t43495 = t5103 * t3336 * t12455;
    let t43497 = t7601 * t11659;
    let t43500 = t2184 * t3308 * t30281;
    let t43503 = t1577 * t10810 * t9296;
    let t43506 = t40062 * t3602 * t8089;
    (t43495, t43497, t43500, t43503, t43506)
}
