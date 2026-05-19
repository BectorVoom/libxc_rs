//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 882/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk882<F: Float>(t1378: F, t147: F, t2331: F, t6056: F, t1952: F, t4579: F, t553: F, t1971: F, t4585: F, t5697: F, t6055: F, t1368: F, t19: F) -> (F, F, F, F, F) {
    let t16422 = F::cast_from(0.67015213385620818113e-4_f64) * t2331 * t147 * t1378 * t6056;
    let t16441 = F::cast_from(0.39507780657818961764e-1_f64) * t1952 * t4579 * t553;
    let t16444 = F::cast_from(0.13871971944573393855e-1_f64) * t5697 * t4585 * t1971;
    let t16446 = F::cast_from(0.2267957317922316773e-1_f64) * t6055 * t1971;
    let t16451 = t1368 * t19;
    (t16422, t16441, t16444, t16446, t16451)
}
