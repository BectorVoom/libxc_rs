//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2989/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2989(t13729: f64, t2782: f64, t4131: f64, t556: f64, t47506: f64, t5722: f64, t1353: f64, t198: f64, t3829: f64, t1868: f64, t4135: f64, t14304: f64, t1450: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49522 = t2782 * t556 * t13729 * t4131;
    let t49528 = t47506 * t5722;
    let t49541 = t198 * t1353;
    let t49544 = t198 * t3829;
    let t49582 = t1868 * t4135;
    let t49647 = t14304 * t1450;
    (t49522, t49528, t49541, t49544, t49582, t49647)
}
