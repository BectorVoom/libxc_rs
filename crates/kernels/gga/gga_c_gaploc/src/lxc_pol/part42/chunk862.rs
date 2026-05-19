//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 862/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk862<F: Float>(t326: F, t45320: F, t825: F, t10930: F, t10931: F, t1457: F, t2645: F, t36516: F, t43464: F, t43467: F, t43470: F, t43522: F) -> (F, F, F, F, F, F, F) {
    let t45343 = F::cast_from(0.18404604457881959845e2_f64) * t825 * t326 * t45320;
    let t45349 = F::cast_from(0.55213813373645879534e2_f64) * t10930 * t10931 * t45320;
    let t45356 = F::cast_from(0.42900587942220512003e1_f64) * t36516 * t1457 * t2645;
    let t45357 = F::cast_from(0.11916829983950142223e0_f64) * t43464;
    let t45358 = F::cast_from(0.11916829983950142223e0_f64) * t43467;
    let t45359 = F::cast_from(0.11916829983950142223e0_f64) * t43470;
    let t45366 = F::cast_from(0.59584149919750711116e-1_f64) * t43522;
    (t45343, t45349, t45356, t45357, t45358, t45359, t45366)
}
