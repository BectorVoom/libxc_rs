//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2354/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354(t13783: f64, t1597: f64, t10237: f64, t2986: f64, t340: f64, t4548: f64, t698: f64, t973: f64, t10186: f64, t10235: f64, t13769: f64, t13770: f64, t13798: f64, t13840: f64, t13852: f64, t13855: f64, t42842: f64, t43028: f64, t43038: f64, t48265: f64, t48269: f64) -> f64 {
    let t48279 = t13783 * t1597;
    let t48281 = t2986 * t48279 * t10237;
    let t48292 = t973 * t698 * t340 * t4548;
    let t48293 = 0.55555555555555555554e-3_f64 * t48292;
    let t48294 = 0.25925925925925925925e-2_f64 * t2986 * t13798 * t48265 - 0.11111111111111111111e-2_f64 * t2986 * t10235 * t48269 + 0.59259259259259259257e-2_f64 * t10186 * t13840 + 0.55555555555555555554e-3_f64 * t43028 + 0.9259259259259259259e-4_f64 * t43038 + 0.29629629629629629629e-2_f64 * t10186 * t13770 - 0.37037037037037037036e-3_f64 * t48281 + 0.22222222222222222221e-2_f64 * t2986 * t13769 * t42842 + 0.44444444444444444443e-2_f64 * t10186 * t13852 + 0.22222222222222222221e-2_f64 * t10186 * t13855 + t48293;
    t48294
}
