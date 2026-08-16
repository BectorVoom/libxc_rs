//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 862/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk862(t326: f64, t45320: f64, t825: f64, t10930: f64, t10931: f64, t1457: f64, t2645: f64, t36516: f64, t43464: f64, t43467: f64, t43470: f64, t43522: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45343 = 0.18404604457881959845e2_f64 * t825 * t326 * t45320;
    let t45349 = 0.55213813373645879534e2_f64 * t10930 * t10931 * t45320;
    let t45356 = 0.42900587942220512003e1_f64 * t36516 * t1457 * t2645;
    let t45357 = 0.11916829983950142223e0_f64 * t43464;
    let t45358 = 0.11916829983950142223e0_f64 * t43467;
    let t45359 = 0.11916829983950142223e0_f64 * t43470;
    let t45366 = 0.59584149919750711116e-1_f64 * t43522;
    (t45343, t45349, t45356, t45357, t45358, t45359, t45366)
}
