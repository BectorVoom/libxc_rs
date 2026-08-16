//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 655/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk655(t185: f64, t5179: f64, t1795: f64, t633: f64, t1730: f64, t1735: f64, t5124: f64, t5128: f64, t5132: f64, t5136: f64, t5140: f64, t5144: f64, t5148: f64, t5151: f64, t5154: f64, t5158: f64, t5160: f64, t5166: f64, t5168: f64, t5170: f64, t5173: f64) -> (f64, f64, f64, f64) {
    let t5181 = 4.0_f64 / 5.0_f64 * t185 * t5179;
    let t5183 = 4.0_f64 / 5.0_f64 * t633 * t1795;
    let t5185 = 4.0_f64 / 5.0_f64 * t1730 * t1735;
    let t5186 = -t5124 + t5128 - t5132 + t5136 - t5140 + t5144 - t5148 - t5151 + t5154 + t5158 - t5160 + t5166 + t5168 + t5170 + t5173 - t5181 + t5183 + t5185;
    (t5181, t5183, t5185, t5186)
}
