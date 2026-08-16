//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2764/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2764(t22025: f64, t2661: f64, t5675: f64, t9934: f64, t6836: f64, t9940: f64, t1353: f64, t13767: f64, t13768: f64, t5591: f64, t21969: f64, t221: f64, t3978: f64, t3979: f64) -> (f64, f64, f64, f64) {
    let t73985 = t2661 * t9934 * t22025 * t5675;
    let t73991 = t9940 * t6836;
    let t73994 = t2661 * t13767 * t73991 * t1353;
    let t73998 = t2661 * t13767 * t13768 * t5591;
    let t74010 = t3978 * t3979 * t221 * t21969;
    (t73985, t73994, t73998, t74010)
}
