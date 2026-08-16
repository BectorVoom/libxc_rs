//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 999/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk999(t1206: f64, t5371: f64, t774: f64, t9986: f64, t1625: f64, t4397: f64, t3348: f64, t12688: f64, t13568: f64, t13570: f64, t13572: f64, t13573: f64, t13574: f64, t13575: f64, t13611: f64, t7929: f64, t7932: f64, t7936: f64, t9839: f64, t9844: f64, t9846: f64, t9848: f64, t9854: f64) -> (f64, f64, f64, f64, f64) {
    let t13793 = t5371 * t1206;
    let t13795 = t9986 * t774 * t13793;
    let t13798 = t1625 * t4397;
    let t13800 = t3348 * t774 * t13798;
    let t13803 = t13568 + t13570 - t13572 - t12688 - t13573 + t13574 - t9839 + t13575 + t9844 + t9846 - t9848 + t7929 - t7932 - t7936 + t9854 + t13611;
    (t13793, t13795, t13798, t13800, t13803)
}
