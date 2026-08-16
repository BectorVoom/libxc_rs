//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1068/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1068(t30673: f64, t4430: f64, t570: f64, t1503: f64, t7329: f64, t1181: f64, t2068: f64, t22048: f64, t604: f64, t33751: f64, t599: f64, t7413: f64) -> (f64, f64, f64, f64, f64) {
    let t34655 = 0.34299214494455789578e-2_f64 * t30673;
    let t34657 = t570 * t4430;
    let t34659 = t7329 * t1503;
    let t34660 = 7.0_f64 / 72.0_f64 * t34659;
    let t34663 = t2068 * t1181 * t604 * t22048;
    let t34667 = t7413 * t1181 * t599 * t33751;
    (t34655, t34657, t34660, t34663, t34667)
}
