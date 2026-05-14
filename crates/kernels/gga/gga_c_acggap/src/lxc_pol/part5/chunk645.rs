//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 645/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk645<F: Float>(t4593: F, t961: F, t1323: F, t3282: F, t1008: F, t1429: F, t1150: F, t3324: F, t3326: F, t3330: F, t3344: F, t3349: F, t335: F, t3358: F, t3364: F, t3368: F, t3373: F, t3376: F, t3380: F, t367: F, t4571: F, t4574: F, t4579: F, t4583: F, t4587: F, t4590: F) -> (F, F, F, F) {
    let t4594 = t4593 * t961;
    let t4597 = t3282 * t1323;
    let t4603 = t1008 * t1429;
    let t4613 = t1150 * t4571 / 8.0 + t335 * t4574 / 24.0 + t1150 * t4579 / 8.0 + t335 * t4583 / 24.0 + t335 * t4587 / 48.0 + t367 * t4590 / 24.0 + t335 * t4594 / 24.0 + t335 * t4597 / 24.0 + 0.10003937560882938627e-2 * t3324 - 0.21437009059034868486e-3 * t3326 - 0.20007875121765877254e-2 * t3330 + 0.85748036236139473944e-2 * t4603 + 0.42874018118069736972e-3 * t3344 + 0.85748036236139473944e-3 * t3349 - 0.34299214494455789578e-2 * t3358 + 0.34299214494455789578e-2 * t3364 - 0.34299214494455789578e-2 * t3368 - 0.40015750243531754508e-2 * t3373 + 0.85748036236139473944e-3 * t3376 + 0.17149607247227894789e-2 * t3380;
    (t4594, t4597, t4603, t4613)
}
