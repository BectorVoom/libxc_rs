//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 598/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk598(t1701: f64, t4100: f64, t4093: f64, t800: f64, t1208: f64, t816: f64, t811: f64, t285: f64, t287: f64, t2724: f64, t820: f64, t2437: f64, t2730: f64, t3796: f64, t3800: f64, t3804: f64, t3808: f64, t3811: f64, t3815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4101 = t1701 * t4100;
    let t4104 = t800 * t4093;
    let t4109 = t816 * t1208;
    let t4110 = t4109 * t811;
    let t4113 = t285 * t287;
    let t4114 = t2724 * t1208;
    let t4115 = t4114 * t820;
    let t4125 = -0.4445200072839506173e-1_f64 * t3796 - 0.4445200072839506173e-1_f64 * t3800 - t2730 + 0.55565000910493827163e-2_f64 * t2437 + 0.55565000910493827163e-2_f64 * t3804 + 0.22226000364197530865e-1_f64 * t3808 - 0.33339000546296296298e-1_f64 * t3811 - 0.33339000546296296298e-1_f64 * t3815;
    (t4101, t4104, t4109, t4110, t4113, t4114, t4115, t4125)
}
