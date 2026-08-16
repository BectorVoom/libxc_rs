//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1202/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1202(t11531: f64, t481: f64, t3262: f64, t3276: f64, t3618: f64, t3270: f64, t10667: f64, t10680: f64, t11587: f64, t37501: f64, t10673: f64, t11591: f64, t37505: f64) -> (f64, f64, f64, f64) {
    let t40416 = t11531 * t481;
    let t40419 = 15.0_f64 / 8.0_f64 * t3262 * t3276 * t40416;
    let t40420 = t3618 * t481;
    let t40421 = t3270 * t40420;
    let t40423 = 3.0_f64 / 2.0_f64 * t10667 * t40421;
    let t40425 = t10680 * t11587 * t37501;
    let t40426 = 0.72042316457491791906e-3_f64 * t40425;
    let t40428 = t10673 * t11591 * t37505;
    (t40419, t40423, t40426, t40428)
}
