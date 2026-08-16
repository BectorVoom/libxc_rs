//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2153/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2153(t10475: f64, t42342: f64, t42345: f64, t2770: f64, t283: f64, t10309: f64, t1041: f64, t10457: f64, t248: f64, t10444: f64, t354: f64, t364: f64, t372: f64) -> (f64, f64, f64, f64) {
    let t43385 = t42342 * t10475 * t42345;
    let t43398 = 1.0_f64 / t283 / t2770;
    let t43406 = t1041 * t248 * t10457 * t10309;
    let t43410 = t354 * t364 * t10444 * t372;
    (t43385, t43398, t43406, t43410)
}
