//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 551/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk551<F: Float>(t291: F, t6: F, t4092: F, t1701: F, t3780: F, t811: F, t1200: F, t1471: F, t820: F, t800: F, t1208: F, t816: F, t285: F, t287: F, t2724: F, t2437: F, t2730: F, t3796: F, t3800: F, t3804: F, t3808: F, t3811: F, t3815: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4093 = t291 * t6;
    let t4094 = t4092 * t4093;
    let t4096 = t1701 * t3780 * t811;
    let t4099 = t1200 * t1471;
    let t4100 = t3780 * t820;
    let t4101 = t1701 * t4100;
    let t4104 = t800 * t4093;
    let t4109 = t816 * t1208;
    let t4110 = t4109 * t811;
    let t4113 = t285 * t287;
    let t4114 = t2724 * t1208;
    let t4115 = t4114 * t820;
    let t4125 = -0.4445200072839506173e-1 * t3796 - 0.4445200072839506173e-1 * t3800 - t2730 + 0.55565000910493827163e-2 * t2437 + 0.55565000910493827163e-2 * t3804 + 0.22226000364197530865e-1 * t3808 - 0.33339000546296296298e-1 * t3811 - 0.33339000546296296298e-1 * t3815;
    (t4094, t4096, t4099, t4101, t4104, t4109, t4110, t4113, t4114, t4115, t4125)
}
