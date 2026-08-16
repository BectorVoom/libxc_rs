//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 795/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk795(t15606: f64, t15609: f64, t15612: f64, t15891: f64, t15894: f64, t15604: f64, t15617: f64, t15621: f64, t15628: f64, t15888: f64, t15897: f64, t15899: f64) -> (f64, f64) {
    let t16336 = 2.0_f64 / 27.0_f64 * t15606;
    let t16337 = 2.0_f64 / 9.0_f64 * t15609;
    let t16338 = t15612 / 9.0_f64;
    let t16342 = t15891 / 3.0_f64;
    let t16343 = 2.0_f64 / 3.0_f64 * t15894;
    let t16345 = -6.0_f64 * t15604 + t16336 - t16337 + t16338 + 2.0_f64 * t15617 + 4.0_f64 * t15621 - t15628 / 3.0_f64 - t15888 + t16342 - t16343 - 8.0_f64 / 9.0_f64 * t15897;
    let t16346 = 2.0_f64 / 9.0_f64 * t15899;
    (t16345, t16346)
}
