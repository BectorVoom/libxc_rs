//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1049/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1049(t1497: f64, t1504: f64, t4911: f64, t4915: f64, t555: f64, t1528: f64, t204: f64, t5063: f64, t148: f64, t1598: f64, t1602: f64, t1527: f64, t5008: f64) -> (f64, f64, f64, f64) {
    let t16280 = 0.61524113149298439947e4_f64 * t555 * t4911 * t1504 * t4915 * t1497;
    let t16283 = 0.14246666666666666666e0_f64 * t204 * t5063 * t1528;
    let t16287 = 0.22911460125803964958e1_f64 * t204 * t148 * t1598 * t1602;
    let t16290 = 0.57895126195293126241e3_f64 * t5008 * t1602 * t1527;
    (t16280, t16283, t16287, t16290)
}
