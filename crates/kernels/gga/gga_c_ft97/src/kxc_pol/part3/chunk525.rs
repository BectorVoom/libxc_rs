//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 525/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk525<F: Float>(t4109: F, t811: F, t285: F, t287: F, t1208: F, t2724: F, t820: F, t2437: F, t2730: F, t3796: F, t3800: F, t3804: F, t3808: F, t3811: F, t3815: F) -> (F, F, F, F, F) {
    let t4110 = t4109 * t811;
    let t4113 = t285 * t287;
    let t4114 = t2724 * t1208;
    let t4115 = t4114 * t820;
    let t4125 = -F::cast_from(0.4445200072839506173e-1_f64) * t3796 - F::cast_from(0.4445200072839506173e-1_f64) * t3800 - t2730 + F::cast_from(0.55565000910493827163e-2_f64) * t2437 + F::cast_from(0.55565000910493827163e-2_f64) * t3804 + F::cast_from(0.22226000364197530865e-1_f64) * t3808 - F::cast_from(0.33339000546296296298e-1_f64) * t3811 - F::cast_from(0.33339000546296296298e-1_f64) * t3815;
    (t4110, t4113, t4114, t4115, t4125)
}
