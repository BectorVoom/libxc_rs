//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2671/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671(t11999: f64, t19826: f64, t11262: f64, t3150: f64, t6307: f64, t11710: f64, t19725: f64, t4892: f64, t15669: f64, t16088: f64, t380: f64, t1045: f64, t4186: f64) -> (f64, f64, f64, f64, f64) {
    let t66024 = t11999 * t19826;
    let t66029 = t3150 * t11262 * t6307;
    let t66043 = t4892 * t11710 * t19725;
    let t66047 = t15669 * t380 * t16088;
    let t66066 = t1045 * t4186;
    (t66024, t66029, t66043, t66047, t66066)
}
