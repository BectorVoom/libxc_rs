//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1094/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1094(t1043: f64, t11967: f64, t1024: f64, t1023: f64, t4060: f64, t1044: f64, t2906: f64, t4063: f64, t1505: f64, t2910: f64, t2914: f64, t1519: f64, t9499: f64) -> (f64, f64, f64, f64, f64) {
    let t11968 = t11967 * t1043;
    let t11970 = 1.0_f64 * t1024 * t11968;
    let t11971 = t4060 * t1023;
    let t11973 = 2.0_f64 * t11971 * t1044;
    let t11975 = 1.0_f64 * t4063 * t2906;
    let t11976 = t1505 * t2910;
    let t11978 = 0.16081979498692535067e2_f64 * t11976 * t2914;
    let t11980 = 1.0_f64 * t9499 * t1519;
    (t11970, t11973, t11975, t11978, t11980)
}
