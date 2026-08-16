//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 759/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk759(t495: f64, t6212: f64, t6211: f64, t6209: f64, t2182: f64, t489: f64, t548: f64, t1572: f64, t1600: f64, t1570: f64, t1632: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6213 = t6212 * t495;
    let t6214 = t6211 * t6213;
    let t6215 = t6209 * t6214;
    let t6217 = t2182 * t489;
    let t6218 = t6217 * t548;
    let t6228 = t1600 * t1572;
    let t6231 = t551 * t1632 * t1570;
    (t6213, t6214, t6215, t6217, t6218, t6228, t6231)
}
