//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1996/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1996(t14066: f64, t225: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64, t1364: f64, t1357: f64, t5775: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14067 = t14066 * t225;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = 0.19514881078765566038e-1_f64 * t3915 * t14079;
    let t14082 = t786 * t5711;
    let t14084 = 0.19514881078765566038e-1_f64 * t14082 * t1364;
    let t14085 = t1357 * t5775;
    let t14087 = 0.10975748638225852664e-1_f64 * t689 * t14085;
    (t14067, t14078, t14079, t14081, t14082, t14084, t14085, t14087)
}
