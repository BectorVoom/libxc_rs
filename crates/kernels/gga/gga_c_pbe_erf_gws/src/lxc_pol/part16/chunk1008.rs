//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1008/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1008<F: Float>(t1125: F, t14024: F, t3139: F, t9026: F, t4028: F, t14007: F, t3261: F, t3291: F, t14015: F, t3253: F, t14064: F, t14063: F, t3179: F, t854: F, t850: F, t8860: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14520 = t1125 * t14024;
    let t14522 = t3139 * t9026;
    let t14523 = t4028 * t14522;
    let t14525 = t14007 * t3261;
    let t14529 = t14007 * t3291;
    let t14531 = t14015 * t3253;
    let t14533 = t1125 * t14064;
    let t14535 = t14063 * t3179;
    let t14536 = t854 * t14535;
    let t14538 = t850 * t8860;
    (t14520, t14522, t14523, t14525, t14529, t14531, t14533, t14535, t14536, t14538)
}
