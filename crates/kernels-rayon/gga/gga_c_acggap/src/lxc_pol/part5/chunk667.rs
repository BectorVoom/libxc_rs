//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 667/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk667(t1089: f64, t1095: f64, t4533: f64, t1451: f64, t997: f64, t506: f64, t839: f64, t368: f64, t1077: f64, t495: f64, t1131: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4535 = t1089 * t1095 * t4533;
    let t4538 = t997 * t1451;
    let t4540 = t506 * t839;
    let t4542 = t1089 * t368 * t4540;
    let t4545 = t495 * t1077;
    let t4547 = t1089 * t368 * t4545;
    let t4550 = t495 * t1131;
    let t4552 = t1089 * t1095 * t4550;
    let t4555 = t495 * t879;
    (t4535, t4538, t4540, t4542, t4547, t4552, t4555)
}
