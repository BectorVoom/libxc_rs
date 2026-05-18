//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1140/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1140<F: Float>(t1165: F, t1432: F, t15947: F, t3361: F, t3375: F, t6351: F, t1173: F, t1176: F, t1180: F, t1181: F, t12536: F, t15469: F, t15479: F, t15483: F, t20417: F, t20422: F, t20430: F, t20433: F, t20441: F, t4643: F, t5270: F, t5852: F) -> F {
    let t20446 = t3361 * t1165 * t15947 * t1432;
    let t20448 = t3375 * t6351;
    let t20450 = F::new(0.34299214494455789578e-2) * t1173 * t20417 * t1176 - F::new(0.20007875121765877254e-2) * t12536 + F::new(0.16006300097412701803e-1) * t20422 + F::new(0.68598428988911579156e-2) * t15469 + F::new(0.34299214494455789578e-2) * t1173 * t1181 * t5852 * t5270 + F::new(0.68598428988911579156e-2) * t15479 - F::new(0.16006300097412701803e-1) * t20430 + F::new(0.17149607247227894789e-2) * t1180 * t1181 * t4643 * t20433 + F::new(0.34299214494455789578e-2) * t20441 - F::new(0.48018900292238105409e-1) * t15483 - F::new(0.68598428988911579156e-2) * t20446 + F::new(0.42874018118069736972e-3) * t20448;
    t20450
}
