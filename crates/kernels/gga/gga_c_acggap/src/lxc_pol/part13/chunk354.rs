//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 354/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk354<F: Float>(t1434: F, t384: F, t1013: F, t1034: F, t1041: F, t1044: F, t1104: F, t1109: F, t1114: F, t1138: F, t1141: F, t1168: F, t1347: F, t1353: F, t1355: F, t1413: F, t1418: F, t1424: F, t1429: F, t397: F, t418: F) -> (F,) {
    let t1435 = t384 * t1434;
    let t1437 = -0.42874018118069736972e-3 * t1013 + t1034 + t1041 - 7.0 / 288.0 * t1044 + 0.85748036236139473944e-3 * t1104 - 0.42874018118069736972e-3 * t1109 + 0.42874018118069736972e-3 * t1114 + 7.0 / 144.0 * t1138 + 7.0 / 288.0 * t1141 + 0.21437009059034868486e-3 * t1168 + 0.17149607247227894789e-2 * t418 * t1347 - 0.42874018118069736972e-3 * t1353 - 0.21437009059034868486e-3 * t1355 - 0.21437009059034868486e-3 * t397 * t1413 - 0.17149607247227894789e-2 * t418 * t1418 + 0.85748036236139473944e-3 * t1424 + 0.42874018118069736972e-2 * t418 * t1429 + 0.42874018118069736972e-3 * t1435;
    (t1437,)
}
