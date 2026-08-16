//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 987/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk987(t13594: f64, t13607: f64, t162: f64, t189: f64, t489: f64, t5343: f64, t724: f64, t1206: f64, t12688: f64, t13568: f64, t13570: f64, t13572: f64, t13573: f64, t13574: f64, t13575: f64, t13576: f64, t198: f64, t4532: f64, t5371: f64, t541: f64, t7929: f64, t7932: f64, t7936: f64, t7945: f64, t9839: f64, t9844: f64, t9846: f64, t9848: f64, t9854: f64) -> (f64, f64, f64, f64) {
    let t13609 = (t13594 + t13607) * t162;
    let t13610 = t13609 * t189;
    let t13611 = t489 * t13610;
    let t13612 = t5343 * t724;
    let t13613 = t489 * t13612;
    let t13614 = 6.0_f64 * t1206 * t198 * t5371 * t541 + 6.0_f64 * t13576 * t4532 - t12688 + t13568 + t13570 - t13572 - t13573 + t13574 + t13575 + t13611 + t13613 + t7929 - t7932 - t7936 + t7945 - t9839 + t9844 + t9846 - t9848 + t9854;
    (t13609, t13611, t13613, t13614)
}
