//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 605/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk605<F: Float>(t336: F, t506: F, t961: F, t1323: F, t3282: F, t1008: F, t1429: F, t1150: F, t3324: F, t3326: F, t3330: F, t3344: F, t3349: F, t335: F, t3358: F, t3364: F, t3368: F, t3373: F, t3376: F, t3380: F, t367: F, t4571: F, t4574: F, t4579: F, t4583: F, t4587: F, t4590: F) -> F {
    let t4593 = t336 * t506;
    let t4594 = t4593 * t961;
    let t4597 = t3282 * t1323;
    let t4603 = t1008 * t1429;
    let t4613 = t1150 * t4571 / F::cast_from(8.0_f64) + t335 * t4574 / F::cast_from(24.0_f64) + t1150 * t4579 / F::cast_from(8.0_f64) + t335 * t4583 / F::cast_from(24.0_f64) + t335 * t4587 / F::cast_from(48.0_f64) + t367 * t4590 / F::cast_from(24.0_f64) + t335 * t4594 / F::cast_from(24.0_f64) + t335 * t4597 / F::cast_from(24.0_f64) + F::cast_from(0.10003937560882938627e-2_f64) * t3324 - F::cast_from(0.21437009059034868486e-3_f64) * t3326 - F::cast_from(0.20007875121765877254e-2_f64) * t3330 + F::cast_from(0.85748036236139473944e-2_f64) * t4603 + F::cast_from(0.42874018118069736972e-3_f64) * t3344 + F::cast_from(0.85748036236139473944e-3_f64) * t3349 - F::cast_from(0.34299214494455789578e-2_f64) * t3358 + F::cast_from(0.34299214494455789578e-2_f64) * t3364 - F::cast_from(0.34299214494455789578e-2_f64) * t3368 - F::cast_from(0.40015750243531754508e-2_f64) * t3373 + F::cast_from(0.85748036236139473944e-3_f64) * t3376 + F::cast_from(0.17149607247227894789e-2_f64) * t3380;
    t4613
}
