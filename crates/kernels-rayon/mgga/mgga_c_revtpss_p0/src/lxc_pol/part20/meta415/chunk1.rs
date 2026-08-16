//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1536/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1536(t11630: f64, t11633: f64, t3172: f64, t11988: f64, t3106: f64, t271: f64, t2852: f64, t41296: f64, t1011: f64, t1012: f64, t1042: f64, t1063: f64, t11634: f64, t11759: f64, t12004: f64, t3101: f64, t3117: f64, t3241: f64, t3253: f64, t39443: f64, t39449: f64, t42615: f64, t43194: f64, t43204: f64, t43207: f64, t4801: f64, t4806: f64, t4892: f64, t4894: f64) -> f64 {
    let t43211 = t11630 * t3172 * t11633;
    let t43215 = t3106 * t11988;
    let t43222 = 1.0_f64 / t271 / t2852;
    let t43223 = t43222 * t41296;
    let t43234 = -0.11433071498151929859e-2_f64 * t1063 * t1042 * t4801 * t43194 + 0.95275595817932748828e-3_f64 * t1063 * t1042 * t4806 * t43194 + 0.38110238327173099531e-3_f64 * t43204 - 0.27439371595564631662e-1_f64 * t43207 * t11634 + 0.34299214494455789578e-2_f64 * t43211 - 0.57927562257303111285e-1_f64 * t12004 * t3101 + 0.20325460441158986416e-2_f64 * t43215 + 0.17149607247227894789e-2_f64 * t4892 * t3117 * t42615 * t4894 + 35.0_f64 / 972.0_f64 * t1011 * t1012 * t43223 * t39443 - t3241 * t11759 / 27.0_f64 + t1011 * t1012 * t3253 * t39449 / 72.0_f64;
    t43234
}
