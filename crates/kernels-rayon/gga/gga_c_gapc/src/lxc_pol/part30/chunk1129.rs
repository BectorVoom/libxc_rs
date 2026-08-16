//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1129/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1129(t2560: f64, t33901: f64, t2568: f64, t11953: f64, t871: f64, t2981: f64, t787: f64, t3752: f64, t869: f64, t11804: f64, t11814: f64, t19210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33902 = t2560 * t33901;
    let t33904 = t2568 * t33901;
    let t33906 = t871 * t11953;
    let t33908 = t33906 * t2981 * t787;
    let t33911 = t869 * t11953 * t3752;
    let t33914 = t11814 * t11804 * t19210;
    (t33902, t33904, t33906, t33908, t33911, t33914)
}
