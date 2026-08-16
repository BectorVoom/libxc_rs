//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1167/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1167(t213: f64, t225: f64, t852: f64, t22986: f64, t23272: f64, t10103: f64, t1880: f64, t6553: f64, t6571: f64, t6552: f64, t6554: f64, t9516: f64) -> (f64, f64, f64) {
    let t82159 = t213 * t852 * t225;
    let t82161 = t22986 * t82159 * t23272;
    let t82165 = t1880 * t6553 * t6571 * t10103;
    let t82169 = t6552 * t6553 * t6554 * t9516;
    (t82161, t82165, t82169)
}
