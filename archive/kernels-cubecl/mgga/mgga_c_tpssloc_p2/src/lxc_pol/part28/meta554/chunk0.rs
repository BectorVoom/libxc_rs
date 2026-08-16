//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1825/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1825<F: Float>(t225: F, t24141: F, t81072: F, t81074: F, t80825: F, t80847: F, t80885: F, t80899: F, t80956: F, t80970: F, t1338: F, t24063: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84433 = t24141 * t225;
    let t84480 = F::cast_from(0.55440370401180965083e0_f64) * t81072;
    let t84481 = F::cast_from(0.3244175520728446583e0_f64) * t81074;
    let t84514 = F::cast_from(0.2034786907144675699e0_f64) * t80825;
    let t84520 = F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t80847;
    let t84533 = F::cast_from(0.67287926823567318088e-4_f64) * t80885;
    let t84536 = F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t80899;
    let t84555 = F::cast_from(0.13958506597733353653e-1_f64) * t80956;
    let t84558 = F::cast_from(0.87474304870637513515e-3_f64) * t80970;
    let t84581 = t1338 * t24063;
    (t84433, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84581)
}
