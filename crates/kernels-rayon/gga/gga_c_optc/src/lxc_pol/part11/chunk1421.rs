//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1421/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1421(t12238: f64, t1471: f64, t15408: f64, t17764: f64, t17835: f64, t2935: f64, t2974: f64, t3059: f64, t402: f64, t5123: f64, t5154: f64, t5158: f64, t5203: f64, t5218: f64, t52351: f64, t58862: f64, t59183: f64, t59186: f64, t59188: f64, t59191: f64, t59193: f64, t59196: f64, t59199: f64, t59202: f64, t59205: f64, t59392: f64, t59404: f64, t8786: f64, t8848: f64) -> f64 {
    let t59428 = -t59183 + t59186 - t59188 - 0.19751789702565206229e-1_f64 * t58862 + t59191 - t59193 + t59196 + t59199 - t59202 - t59205 - 0.3109e-1_f64 * (t59392 + t59404) * t402 + 0.38597619813444837568e3_f64 * t12238 * t17764 - 0.11579285944033451271e4_f64 * t8786 * t5158 * t5154 - 8.0_f64 * t2935 * t17835 * t1471 + 0.1286587327114827919e3_f64 * t2974 * t52351 * t1471 + 0.12414802127193579148e5_f64 * t8848 * t15408 * t5154 + 0.21053604230838734656e2_f64 * t3059 * t5203 * t5218 + 36.0_f64 * t2974 * t5123 * t5154;
    t59428
}
