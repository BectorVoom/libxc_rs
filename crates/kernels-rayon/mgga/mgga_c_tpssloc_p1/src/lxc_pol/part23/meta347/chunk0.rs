//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1140/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1140(t40227: f64, t12132: f64, t592: f64, t68: f64, t6924: f64, t1336: f64, t1339: f64, t2691: f64, t10021: f64, t154: f64, t59: f64, t3749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40228 = 72.0_f64 * t40227;
    let t40230 = 16.0_f64 * t592 * t12132;
    let t40253 = t68 * t6924;
    let t40281 = t1336 * t1339 * t2691;
    let t40341 = t59 * t10021 * t154;
    let t40343 = 0.99537037037037037035e-1_f64 * t40341 * t3749;
    (t40228, t40230, t40253, t40281, t40341, t40343)
}
