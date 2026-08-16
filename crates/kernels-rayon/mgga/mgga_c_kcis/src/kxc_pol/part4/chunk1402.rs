//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1402/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1402(t18217: f64, t6176: f64, t1369: f64, t2470: f64, t6164: f64, t1599: f64, t12615: f64, t12664: f64, t18184: f64, t18188: f64, t18192: f64, t18197: f64, t18201: f64, t18205: f64, t18213: f64, t4435: f64, t4439: f64, t4442: f64, t4451: f64, t6141: f64) -> f64 {
    let t18218 = t6176 * t18217;
    let t18221 = t2470 * t1369;
    let t18222 = t18221 * t6164;
    let t18223 = t1599 * t18222;
    let t18225 = 7.0_f64 / 1296.0_f64 * t4439 * t18184 - t4439 * t18188 / 108.0_f64 + t18192 * t4442 / 108.0_f64 + t1599 * t18197 / 48.0_f64 + t1599 * t18201 / 96.0_f64 - t18205 - t6141 * t4451 / 216.0_f64 - t6141 * t4435 / 162.0_f64 + t18213 - t12615 / 576.0_f64 + t12664 / 288.0_f64 - t1599 * t18218 / 32.0_f64 + 7.0_f64 / 864.0_f64 * t18223;
    t18225
}
