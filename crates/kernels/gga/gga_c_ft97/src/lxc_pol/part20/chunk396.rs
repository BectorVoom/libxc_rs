//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 396/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk396<F: Float>(t1208: F, t2724: F, t820: F, t2437: F, t2730: F, t3796: F, t3800: F, t3804: F, t3808: F, t3811: F, t3815: F) -> (F, F, F) {
    let t4114 = t2724 * t1208;
    let t4115 = t4114 * t820;
    let t4125 = -0.4445200072839506173e-1 * t3796 - 0.4445200072839506173e-1 * t3800 - t2730 + 0.55565000910493827163e-2 * t2437 + 0.55565000910493827163e-2 * t3804 + 0.22226000364197530865e-1 * t3808 - 0.33339000546296296298e-1 * t3811 - 0.33339000546296296298e-1 * t3815;
    (t4114, t4115, t4125)
}
