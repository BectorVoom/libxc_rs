//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1297/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1297(t3379: f64, t5618: f64, t1410: f64, t2937: f64, t406: f64, t16899: f64, t6324: f64, t3409: f64, t6086: f64, t1101: f64, t1165: f64, t1889: f64, t4282: f64) -> (f64, f64, f64, f64, f64) {
    let t24110 = t3379 * t5618;
    let t24112 = t2937 * t1410;
    let t24113 = t24112 * t406;
    let t24128 = t16899 * t6324;
    let t24130 = t3409 * t6086;
    let t24138 = t4282 * t1165 * t1889 * t1101;
    (t24110, t24113, t24128, t24130, t24138)
}
