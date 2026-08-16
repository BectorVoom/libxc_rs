//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 767/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk767(t1030: f64, t8986: f64, t3076: f64, t1795: f64, t3104: f64, t1636: f64, t189: f64, t185: f64, t1771: f64, t1723: f64, t8770: f64, t654: f64, t8768: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8987 = t1030 * t8986;
    let t8988 = t8987 * t3076;
    let t8990 = t3104 * t1795;
    let t8992 = t189 * t1636;
    let t8993 = t185 * t8992;
    let t8994 = t8993 * t1771;
    let t8996 = t8770 * t1723;
    let t8998 = t654 * t8768;
    (t8987, t8988, t8990, t8992, t8994, t8996, t8998)
}
