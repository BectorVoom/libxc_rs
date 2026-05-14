//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 695/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk695<F: Float>(t7725: F, t7737: F, t7739: F, t7742: F, t7747: F, t7775: F, t7781: F, t7787: F, t7800: F, t7802: F, t7805: F, t7849: F, t7853: F, t7862: F, t394: F, t633: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8249 = 0.21437009059034868486e-3 * t7725;
    let t8252 = 0.17149607247227894789e-2 * t7737;
    let t8253 = 0.85748036236139473944e-3 * t7739;
    let t8254 = 0.17149607247227894789e-2 * t7742;
    let t8257 = 0.80031500487063509014e-2 * t7747;
    let t8268 = 0.19055119163586549766e-2 * t7775;
    let t8269 = 0.90035438047946447644e-2 * t7781;
    let t8271 = 0.13208198761633743869e-1 * t7787;
    let t8275 = 0.28582678745379824648e-3 * t7800;
    let t8276 = 0.31448092289604152069e-3 * t7802;
    let t8278 = 0.41930789719472202758e-3 * t7805;
    let t8291 = 77.0 / 864.0 * t7849;
    let t8292 = 35.0 / 216.0 * t7853;
    let t8294 = t7862 / 192.0;
    let t8306 = t394 * t633;
    (t8249, t8252, t8253, t8254, t8257, t8268, t8269, t8271, t8275, t8276, t8278, t8291, t8292, t8294, t8306)
}
