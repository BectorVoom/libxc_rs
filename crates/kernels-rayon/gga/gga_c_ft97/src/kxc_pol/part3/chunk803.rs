//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 803/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk803(t16345: f64, t16357: f64, t16365: f64, t16478: f64, t488: f64, t83: f64, t379: f64, t4572: f64, t8557: f64, t1882: f64, t4553: f64, t15604: f64, t15606: f64, t15609: f64, t15612: f64, t15617: f64, t15621: f64, t15628: f64, t15888: f64, t15891: f64, t15894: f64, t15897: f64) -> (f64, f64, f64, f64, f64) {
    let t16480 = t16345 + t16357 + t16365 + t16478;
    let t16481 = t488 * t16480;
    let t16482 = t83 * t16481;
    let t16485 = t4572 * t379;
    let t16486 = t8557 * t16485;
    let t16490 = t1882 * t4553;
    let t16503 = -2.0_f64 * t15604 + 2.0_f64 / 81.0_f64 * t15606 - 2.0_f64 / 27.0_f64 * t15609 + t15612 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t15617 + 4.0_f64 / 3.0_f64 * t15621 - t15628 / 9.0_f64 - t15888 / 3.0_f64 + t15891 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t15894 - 8.0_f64 / 27.0_f64 * t15897;
    (t16481, t16482, t16486, t16490, t16503)
}
