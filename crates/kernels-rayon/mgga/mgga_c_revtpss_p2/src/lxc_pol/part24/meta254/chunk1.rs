//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1022/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1022(t16088: f64, t16094: f64, t1651: f64, t3181: f64, t11150: f64, t11144: f64, t11852: f64, t1655: f64, t697: f64, t1011: f64, t372: f64, t4806: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16095 = t16094 * t16088;
    let t16170 = t3181 * t1651;
    let t16199 = t3181 * t11150;
    let t16208 = t11852 * t11144;
    let t16219 = t697 * t1655;
    let t16220 = t1011 * t16219;
    let t16222 = t372 * t4806;
    (t16095, t16170, t16199, t16208, t16220, t16222)
}
