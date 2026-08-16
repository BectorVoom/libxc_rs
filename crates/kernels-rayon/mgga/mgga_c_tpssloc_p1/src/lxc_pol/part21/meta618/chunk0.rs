//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2394/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2394(t1369: f64, t40059: f64, t12345: f64, t3876: f64, t22843: f64, t241: f64, t67: f64, t3872: f64, t10021: f64, t1336: f64, t1339: f64, t1354: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40060 = t40059 * t1369;
    let t40065 = t12345 * t3876;
    let t40070 = t241 * t22843 * t67;
    let t40079 = t12345 * t3872;
    let t40123 = t1336 * t1339 * t10021;
    let t40124 = t40123 * t1354;
    (t40060, t40065, t40070, t40079, t40123, t40124)
}
