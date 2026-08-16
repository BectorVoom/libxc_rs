//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1232/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1232(t184: f64, t20217: f64, t120: f64, t20856: f64, t46657: f64, t5593: f64, t20852: f64, t13258: f64, t20983: f64, t20974: f64, t9638: f64, t20891: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67469 = t184 * t20217;
    let t67607 = t120 * t20856;
    let t67612 = t46657 * t5593;
    let t67620 = t120 * t20852;
    let t67625 = t13258 * t20983;
    let t67637 = t9638 * t20974;
    let t67639 = t9638 * t20891;
    (t67469, t67607, t67612, t67620, t67625, t67637, t67639)
}
