//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 604/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk604(t1039: f64, t3775: f64, t1029: f64, t997: f64, t1032: f64, t993: f64, t1205: f64, t3266: f64, t386: f64, t388: f64, t384: f64, t1103: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3777 = 0.12862205435420921092e-2_f64 * t3775 * t1039;
    let t3778 = t997 * t1029;
    let t3782 = 0.30011812682648815881e-2_f64 * t1032 * t993;
    let t3783 = t997 * t1205;
    let t3786 = t386 * t3266 * t388;
    let t3787 = t384 * t3786;
    let t3793 = t1032 * t1103;
    (t3777, t3778, t3782, t3783, t3786, t3787, t3793)
}
