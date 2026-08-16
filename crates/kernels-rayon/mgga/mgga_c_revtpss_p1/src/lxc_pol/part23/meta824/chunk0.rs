//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2677/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2677(t11773: f64, t4954: f64, t1011: f64, t6284: f64, t697: f64, t19900: f64, t3241: f64, t19477: f64, t3153: f64, t15905: f64, t56017: f64, t55899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66542 = t4954 * t11773;
    let t66547 = t1011 * t697 * t6284;
    let t66551 = t3241 * t19900;
    let t66565 = t19477 * t3153;
    let t66621 = t56017 * t15905;
    let t66624 = t55899 * t15905;
    (t66542, t66547, t66551, t66565, t66621, t66624)
}
