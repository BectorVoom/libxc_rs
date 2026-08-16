//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 730/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk730(t205: f64, t8690: f64, t1587: f64, t1720: f64, t3104: f64, t1707: f64, t3103: f64, t1504: f64, t126: f64, t417: f64, t581: f64, t3105: f64) -> (f64, f64, f64, f64, f64) {
    let t8691 = t8690 * t205;
    let t8693 = t1720 * t1587;
    let t8694 = t3104 * t8693;
    let t8696 = t1707 * t3103;
    let t8697 = t1720 * t1504;
    let t8698 = t8696 * t8697;
    let t8700 = t126 * t417;
    let t8701 = t581 * t8700;
    let t8702 = t8701 * t3105;
    (t8691, t8694, t8698, t8700, t8702)
}
