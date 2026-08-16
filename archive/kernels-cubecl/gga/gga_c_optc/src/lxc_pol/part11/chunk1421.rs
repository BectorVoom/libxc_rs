//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1421/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1421<F: Float>(t12238: F, t1471: F, t15408: F, t17764: F, t17835: F, t2935: F, t2974: F, t3059: F, t402: F, t5123: F, t5154: F, t5158: F, t5203: F, t5218: F, t52351: F, t58862: F, t59183: F, t59186: F, t59188: F, t59191: F, t59193: F, t59196: F, t59199: F, t59202: F, t59205: F, t59392: F, t59404: F, t8786: F, t8848: F) -> F {
    let t59428 = -t59183 + t59186 - t59188 - F::cast_from(0.19751789702565206229e-1_f64) * t58862 + t59191 - t59193 + t59196 + t59199 - t59202 - t59205 - F::cast_from(0.3109e-1_f64) * (t59392 + t59404) * t402 + F::cast_from(0.38597619813444837568e3_f64) * t12238 * t17764 - F::cast_from(0.11579285944033451271e4_f64) * t8786 * t5158 * t5154 - F::cast_from(8.0_f64) * t2935 * t17835 * t1471 + F::cast_from(0.1286587327114827919e3_f64) * t2974 * t52351 * t1471 + F::cast_from(0.12414802127193579148e5_f64) * t8848 * t15408 * t5154 + F::cast_from(0.21053604230838734656e2_f64) * t3059 * t5203 * t5218 + F::cast_from(36.0_f64) * t2974 * t5123 * t5154;
    t59428
}
