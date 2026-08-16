//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1299/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1299(t1181: f64, t12936: f64, t4643: f64, t5099: f64, t1891: f64, t3670: f64, t1881: f64, t3237: f64, t1137: f64, t6301: f64, t6305: f64, t3621: f64, t6389: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24145 = t12936 * t1181 * t4643 * t5099;
    let t24147 = t3670 * t1891;
    let t24149 = t3237 * t1881;
    let t24151 = t1137 * t6301;
    let t24153 = t1137 * t6305;
    let t24155 = t3621 * t6389;
    (t24145, t24147, t24149, t24151, t24153, t24155)
}
