//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 924/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk924(t1368: f64, t1464: f64, t285: f64, t168: f64, t18344: f64, t286: f64, t147: f64, t18049: f64, t281: f64, t545: f64, t5984: f64, t159: f64, t4259: f64) -> (f64, f64, f64, f64, f64) {
    let t19107 = 0.81358876250083374227e-2_f64 * t1464 * t1368 * t285;
    let t19121 = 0.91063310497738755577e0_f64 * t168 * t18344 * t286;
    let t19157 = 0.11974234010254609094e-1_f64 * t281 * t147 * t18049 * t285;
    let t19169 = 0.26861343269868796571e-1_f64 * t5984 * t545 * t285;
    let t19174 = 0.10943113336969376162e-5_f64 * t4259 * t147 * t159 * t285;
    (t19107, t19121, t19157, t19169, t19174)
}
