//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 868/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk868<F: Float>(t11849: F, t2628: F, t43646: F, t43652: F, t43657: F, t43660: F, t43679: F, t43681: F, t11848: F, t2021: F, t7372: F, t11576: F, t123: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45441 = t11849 * t2628;
    let t45442 = F::new(0.29792074959875355558e-1) * t45441;
    let t45451 = F::new(0.17875244975925213335e0) * t43646;
    let t45453 = F::new(0.30674340763136599741e1) * t43652;
    let t45454 = F::new(0.20449560508757733161e1) * t43657;
    let t45457 = F::new(0.34082600847929555269e0) * t43660;
    let t45458 = F::new(0.59584149919750711116e-1) * t43679;
    let t45459 = F::new(0.71500979903700853339e0) * t43681;
    let t45463 = t2021 * t11848 * t7372;
    let t45464 = F::new(0.14896037479937677779e-1) * t45463;
    let t45466 = t11576 * t123 * t883;
    (t45442, t45451, t45453, t45454, t45457, t45458, t45459, t45464, t45466)
}
