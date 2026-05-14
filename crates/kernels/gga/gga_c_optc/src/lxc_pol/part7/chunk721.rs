//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 721/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk721<F: Float>(t1000: F, t7244: F, t914: F, t1015: F, t2337: F, t2360: F, t2364: F, t2433: F, t2544: F, t355: F, t4038: F, t7175: F, t7180: F, t7183: F, t7186: F, t7188: F, t7195: F, t7199: F, t7204: F, t7208: F, t7210: F, t7215: F, t7219: F, t7224: F, t7230: F, t7235: F, t7240: F, t999: F) -> (F, F, F) {
    let t7245 = t1000 * t7244;
    let t7246 = t914 * t7245;
    let t7249 = -100.0 / 27.0 * t2433 * t7175 - t4038 * t7180 + 50.0 / 9.0 * t7183 * t1015 - 50.0 / 3.0 * t7186 - 50.0 * t7188 * t1015 - 616.0 / 27.0 * t355 * t7195 + 44.0 / 9.0 * t7199 + t7204 - 100.0 / 81.0 * t7208 + 100.0 / 27.0 * t7210 + 100.0 / 81.0 * t7215 + 20000.0 / 81.0 * t7219 * t7224 - 380000.0 / 81.0 * t7230 * t2337 + 20000.0 / 81.0 * t7235 - 4.0 / 3.0 * t2364 * t2544 + t7240 / 6.0 + t2360 * t2544 / 2.0 + t999 * t7246 / 6.0;
    (t7245, t7246, t7249)
}
