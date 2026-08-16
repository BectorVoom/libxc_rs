//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1135/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1135(t13847: f64, t13848: f64, t1399: f64, t9816: f64, t2713: f64, t3964: f64, t5617: f64, t1872: f64, t3829: f64, t800: f64, t124: f64, t13716: f64) -> (f64, f64, f64, f64) {
    let t14005 = t13847 * t13848 * t1399;
    let t14007 = 0.25410001404642664112e-4_f64 * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    let t14016 = t800 * t1872 * t3829;
    let t14019 = t124 * t13716;
    (t14007, t14013, t14016, t14019)
}
