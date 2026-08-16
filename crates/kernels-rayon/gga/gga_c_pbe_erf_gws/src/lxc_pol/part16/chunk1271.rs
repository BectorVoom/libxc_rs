//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1271/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1271(t3065: f64, t3167: f64, t2134: f64, t3253: f64, t51255: f64, t14099: f64, t863: f64, t885: f64, t338: f64, t8886: f64, t1125: f64, t51221: f64) -> (f64, f64, f64, f64) {
    let t54084 = t3065 * t3167;
    let t54085 = t2134 * t54084;
    let t54087 = t51255 * t3253;
    let t54090 = t863 * t14099 * t885;
    let t54092 = t54090 * t338 * t8886;
    let t54094 = t1125 * t51221;
    (t54085, t54087, t54092, t54094)
}
