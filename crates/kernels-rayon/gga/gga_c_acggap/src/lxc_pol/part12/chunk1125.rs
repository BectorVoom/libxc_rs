//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1125/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1125(t1345: f64, t30148: f64, t30154: f64, t7842: f64, t34569: f64, t8465: f64, t5281: f64, t7561: f64, t1992: f64, t30692: f64, t5720: f64, t30364: f64, t5147: f64) -> (f64, f64, f64, f64, f64) {
    let t35995 = t30154 * t7842 * t30148 * t1345;
    let t35997 = t34569 * t8465;
    let t35999 = t7561 * t5281;
    let t36004 = t30692 * t7842 * t1992 * t5720;
    let t36006 = t30364 * t5147;
    (t35995, t35997, t35999, t36004, t36006)
}
