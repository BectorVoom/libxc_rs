//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2576/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2576(t44620: f64, t461: f64, t60: f64, t15394: f64, t1714: f64, t3439: f64, t3447: f64, t4724: f64, t697: f64, t11590: f64, t15376: f64, t11554: f64, t1706: f64) -> (f64, f64, f64, f64, f64) {
    let t52096 = t60 * t44620 * t461;
    let t52100 = t15394 * t1714;
    let t52109 = t3447 * t697 * t3439 * t461 * t4724;
    let t52110 = 0.24691358024691358024e-3_f64 * t52109;
    let t52122 = t15376 * t11590;
    let t52124 = t1706 * t11554;
    (t52096, t52100, t52110, t52122, t52124)
}
