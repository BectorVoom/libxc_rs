//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1053/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1053<F: Float>(t36380: F, t2016: F, t8747: F, t31879: F, t7637: F, t8571: F, t1998: F, t5251: F, t1967: F, t8566: F, t4557: F, t31859: F, t31864: F, t31868: F, t31870: F, t31872: F, t36365: F, t36368: F, t36370: F, t36373: F, t36374: F, t36378: F) -> (F,) {
    let t36381 = 7.0 / 144.0 * t36380;
    let t36382 = t2016 * t8747;
    let t36383 = 0.28015625e-1 * t36382;
    let t36385 = 0.17149607247227894789e-2 * t31879;
    let t36386 = t7637 * t8571;
    let t36388 = t1998 * t5251;
    let t36389 = 0.34299214494455789578e-2 * t36388;
    let t36390 = t1967 * t8566;
    let t36391 = 0.37737710747524982482e-2 * t36390;
    let t36392 = t1998 * t4557;
    let t36394 = 0.42874018118069736972e-3 * t31859 + t36365 + t36368 + 0.85748036236139473944e-3 * t31864 - 0.17149607247227894789e-2 * t36370 - t36373 - 0.17149607247227894789e-2 * t36374 + t31868 - t36378 + t31870 / 16.0 + t36381 + t36383 - 7.0 / 288.0 * t31872 - t36385 + 0.27953859812981468505e-2 * t36386 + t36389 + t36391 + 0.17149607247227894789e-2 * t36392;
    (t36394,)
}
