//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1189/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1189(t26358: f64, t48291: f64, t48295: f64, t48299: f64, t48303: f64, t48305: f64, t48306: f64, t48307: f64, t48309: f64, t48310: f64, t48311: f64, t33193: f64, t33196: f64, t48313: f64, t48315: f64, t48316: f64, t48318: f64, t48320: f64, t48330: f64, t48359: f64, t48363: f64, t48367: f64) -> (f64, f64) {
    let t48686 = -t48291 - t48295 + t48299 + t48303 + t48305 + 0.13418091289332405787e0_f64 * t26358 + t48306 + t48307 - t48309 + t48310 - t48311;
    let t48689 = -t48313 + t48315 + t48316 + 0.12985249634837812052e1_f64 * t33193 + 0.67090456446662028936e-1_f64 * t33196 + t48318 + t48320 + t48330 + t48359 - t48363 + t48367;
    (t48686, t48689)
}
