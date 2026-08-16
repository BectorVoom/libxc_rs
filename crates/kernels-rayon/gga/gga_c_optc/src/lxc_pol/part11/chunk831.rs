//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 831/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk831(t1220: f64, t16094: f64, t4230: f64, t4539: f64, t1570: f64, t4275: f64, t1199: f64, t5454: f64, t12966: f64, t1256: f64, t4599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16095 = t1220 * t16094;
    let t16097 = t4230 * t4539;
    let t16099 = t1570 * t4275;
    let t16135 = t5454 * t1199;
    let t16220 = 12.0_f64 * t12966;
    let t16221 = t4599 * t1256;
    (t16095, t16097, t16099, t16135, t16220, t16221)
}
