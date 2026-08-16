//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 598/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk598(t1096: f64, t1165: f64, t4267: f64, t1173: f64, t1180: f64, t1531: f64, t3199: f64, t3209: f64, t3396: f64, t367: f64, t4402: f64, t4406: f64, t4410: f64, t4414: f64, t4419: f64, t4423: f64, t4427: f64, t4430: f64, t4434: f64, t4439: f64, t4443: f64, t4447: f64, t4450: f64, t4452: f64, t4456: f64, t4459: f64, t4462: f64, t4463: f64, t4465: f64) -> (f64, f64) {
    let t4469 = t1165 * t4267 * t1096;
    let t4472 = 0.42874018118069736972e-3_f64 * t1180 * t4402 - 0.42874018118069736972e-3_f64 * t1180 * t4406 + 0.85748036236139473944e-3_f64 * t1173 * t4410 - 0.21437009059034868486e-3_f64 * t1180 * t4414 - 0.10289764348336736874e-1_f64 * t3396 * t4419 + t4423 + t4427 - 0.17149607247227894789e-2_f64 * t3199 - t3209 - t367 * t4430 / 96.0_f64 - t367 * t4434 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t1180 * t4439 - 0.21437009059034868486e-3_f64 * t1180 * t4443 + 0.85748036236139473944e-3_f64 * t1173 * t4447 - 0.12862205435420921092e-2_f64 * t4450 * t4452 + 0.12862205435420921092e-2_f64 * t1531 * t4456 + 0.42874018118069736972e-3_f64 * t4459 + t4462 + 0.17149607247227894789e-1_f64 * t4463 * t4465 - 0.17149607247227894789e-1_f64 * t4463 * t4469;
    (t4469, t4472)
}
