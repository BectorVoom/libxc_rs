//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1192/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1192(t1795: f64, t1983: f64, t2095: f64, t1967: f64, t9577: f64, t1426: f64, t2085: f64, t22099: f64, t598: f64, t1089: f64, t4643: f64, t8564: f64) -> (f64, f64, f64, f64) {
    let t40425 = t2095 * t1983 * t1795;
    let t40427 = t1967 * t9577;
    let t40431 = t598 * t1426 * t22099 * t2085;
    let t40436 = t598 * t1089 * t4643 * t8564;
    (t40425, t40427, t40431, t40436)
}
