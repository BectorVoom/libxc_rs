//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 600/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk600<F: Float>(t1096: F, t1165: F, t4267: F, t1173: F, t1180: F, t1531: F, t3199: F, t3209: F, t3396: F, t367: F, t4402: F, t4406: F, t4410: F, t4414: F, t4419: F, t4423: F, t4427: F, t4430: F, t4434: F, t4439: F, t4443: F, t4447: F, t4450: F, t4452: F, t4456: F, t4459: F, t4462: F, t4463: F, t4465: F) -> (F, F) {
    let t4469 = t1165 * t4267 * t1096;
    let t4472 = F::new(0.42874018118069736972e-3) * t1180 * t4402 - F::new(0.42874018118069736972e-3) * t1180 * t4406 + F::new(0.85748036236139473944e-3) * t1173 * t4410 - F::new(0.21437009059034868486e-3) * t1180 * t4414 - F::new(0.10289764348336736874e-1) * t3396 * t4419 + t4423 + t4427 - F::new(0.17149607247227894789e-2) * t3199 - t3209 - t367 * t4430 / F::new(96.0) - t367 * t4434 / F::new(96.0) + F::new(0.42874018118069736972e-3) * t1180 * t4439 - F::new(0.21437009059034868486e-3) * t1180 * t4443 + F::new(0.85748036236139473944e-3) * t1173 * t4447 - F::new(0.12862205435420921092e-2) * t4450 * t4452 + F::new(0.12862205435420921092e-2) * t1531 * t4456 + F::new(0.42874018118069736972e-3) * t4459 + t4462 + F::new(0.17149607247227894789e-1) * t4463 * t4465 - F::new(0.17149607247227894789e-1) * t4463 * t4469;
    (t4469, t4472)
}
