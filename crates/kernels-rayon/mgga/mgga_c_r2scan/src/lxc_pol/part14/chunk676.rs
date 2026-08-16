//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 676/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk676(t1411: f64, t732: f64, t1376: f64, t457: f64, t41: f64, t1524: f64, t1384: f64, t4811: f64, t4816: f64, t234: f64, t105: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t5029 = t732 * t1411;
    let t5031 = t1376 * t457;
    let t5032 = t41 * t5031;
    let t5034 = t732 * t1524;
    let t5037 = t4816 * t4811 * t1384;
    let t5038 = t234 * t5037;
    let t5039 = 0.10389515463408878255e3_f64 * t5038;
    let t5052 = 1.0_f64 / t488 / t105;
    (t5029, t5032, t5034, t5039, t5052)
}
