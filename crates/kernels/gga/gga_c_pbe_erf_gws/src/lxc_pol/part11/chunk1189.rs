//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1189/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1189<F: Float>(t26358: F, t48291: F, t48295: F, t48299: F, t48303: F, t48305: F, t48306: F, t48307: F, t48309: F, t48310: F, t48311: F, t33193: F, t33196: F, t48313: F, t48315: F, t48316: F, t48318: F, t48320: F, t48330: F, t48359: F, t48363: F, t48367: F) -> (F, F) {
    let t48686 = -t48291 - t48295 + t48299 + t48303 + t48305 + F::cast_from(0.13418091289332405787e0_f64) * t26358 + t48306 + t48307 - t48309 + t48310 - t48311;
    let t48689 = -t48313 + t48315 + t48316 + F::cast_from(0.12985249634837812052e1_f64) * t33193 + F::cast_from(0.67090456446662028936e-1_f64) * t33196 + t48318 + t48320 + t48330 + t48359 - t48363 + t48367;
    (t48686, t48689)
}
