//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1124/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1124(t10781: f64, t7970: f64, t2553: f64, t37764: f64, t11693: f64, t6205: f64, t7373: f64, t10776: f64, t3308: f64, t7990: f64, t1058: f64, t1060: f64, t2201: f64, t7290: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39577 = t10781 * t7970;
    let t39579 = t37764 * t2553;
    let t39581 = t6205 * t11693;
    let t39583 = t10781 * t7373;
    let t39586 = t10776 * t3308 * t7990;
    let t39599 = t2201 * t1058 * t1060 * t7290;
    (t39577, t39579, t39581, t39583, t39586, t39599)
}
