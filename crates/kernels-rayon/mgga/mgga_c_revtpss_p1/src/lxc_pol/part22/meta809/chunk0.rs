//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2911/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2911(t1386: f64, t2237: f64, t2482: f64, t4021: f64, t235: f64, t46475: f64, t4000: f64, t596: f64, t10003: f64, t4059: f64, t9909: f64, t72: f64, t9940: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47198 = t2482 * t1386 * t2237;
    let t47199 = t47198 * t4021;
    let t47201 = t46475 * t235;
    let t47215 = t2482 * t4000 * t596;
    let t47216 = t47215 * t10003;
    let t47229 = t9909 * t4059;
    let t47247 = t9940 * t72;
    (t47198, t47199, t47201, t47215, t47216, t47229, t47247)
}
