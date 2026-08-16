//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 747/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk747(t609: f64, t1610: f64, t4456: f64, t286: f64, t4390: f64, t1608: f64, t1599: f64, t4424: f64, t4427: f64, t4430: f64, t4435: f64, t4439: f64, t4442: f64, t4447: f64, t4451: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t614 = 0.0_f64 < t609;
    let t4457 = t1610 * t1610;
    let t4458 = t4456 * t4457;
    let t4459 = t286 * t4458;
    let t4463 = piecewise3(t614, t4390, -t4390);
    let t4464 = t1608 * t4463;
    let t4465 = t286 * t4464;
    let t4468 = -t4424 + t4427 / 864.0_f64 - t4430 / 288.0_f64 + t1599 * t4435 / 432.0_f64 - t4439 * t4442 / 288.0_f64 - t1599 * t4447 / 288.0_f64 + t1599 * t4451 / 576.0_f64 + t1599 * t4459 / 96.0_f64 - t1599 * t4465 / 192.0_f64;
    (t4457, t4458, t4459, t4463, t4464, t4465, t4468)
}
