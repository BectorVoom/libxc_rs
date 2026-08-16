//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2359/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2359(t10346: f64, t13813: f64, t13882: f64, t13887: f64, t1593: f64, t1597: f64, t2960: f64, t343: f64, t42554: f64, t4546: f64, t48394: f64, t48397: f64, t48402: f64, t48407: f64, t48417: f64, t48421: f64, t973: f64) -> f64 {
    let t48423 = 0.74074074074074074072e-3_f64 * t48394 + 0.10288065843621399177e-3_f64 * t48397 + 0.13333333333333333333e-1_f64 * t2960 * t13813 - 0.16666666666666666666e-2_f64 * t48402 + 0.66666666666666666666e-2_f64 * t2960 * t13882 - 0.83333333333333333331e-3_f64 * t48407 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t1597 * t10346 * t343 + 0.66666666666666666666e-2_f64 * t2960 * t13887 - 0.83333333333333333331e-3_f64 * t48417 - 0.12674897119341563785e-1_f64 * t42554 * t1593 + 0.27160493827160493826e-2_f64 * t48421;
    t48423
}
