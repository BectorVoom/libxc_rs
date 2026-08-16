//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 996/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk996(t1446: f64, t3237: f64, t4724: f64, t997: f64, t5113: f64, t3670: f64, t1032: f64, t4503: f64, t4625: f64, t1181: f64, t16507: f64, t3361: f64, t4267: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16637 = t3237 * t1446;
    let t16639 = t997 * t4724;
    let t16641 = t997 * t5113;
    let t16644 = t3670 * t1446;
    let t16646 = t1032 * t4503;
    let t16648 = t1032 * t4625;
    let t16663 = t3361 * t1181 * t4267 * t16507;
    (t16637, t16639, t16641, t16644, t16646, t16648, t16663)
}
