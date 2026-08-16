//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1388/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1388(t5133: f64, t2958: f64, t5126: f64, t26424: f64, t2941: f64, t26266: f64, t1045: f64, t58753: f64, t1450: f64, t52528: f64, t52533: f64, t14984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t58757 = t5133 * t5133;
    let t58758 = t2958 * t58757;
    let t58760 = t5126 * t5126;
    let t58761 = t26424 * t58760;
    let t58763 = t2941 * t58757;
    let t58765 = t26266 * t58760;
    let t58770 = t1045 * t58753;
    let t58774 = t52528 * t1450;
    let t58776 = t52533 * t1450;
    let t58778 = t14984 * t5133;
    (t58758, t58761, t58763, t58765, t58770, t58774, t58776, t58778)
}
