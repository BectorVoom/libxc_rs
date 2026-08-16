//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1060/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1060(t281: f64, t285: f64, t4576: f64, t535: f64, t147: f64, t18049: f64, t520: f64, t5621: f64, t5624: f64, t159: f64, t18068: f64, t545: f64, t5984: f64) -> (f64, f64, f64, f64, f64) {
    let t19152 = t281 * t535 * t4576 * t285;
    let t19157 = 0.11974234010254609094e-1_f64 * t281 * t147 * t18049 * t285;
    let t19160 = t5621 * t520;
    let t19161 = t19160 * t5624;
    let t19165 = t18068 * t159 * t285;
    let t19169 = 0.26861343269868796571e-1_f64 * t5984 * t545 * t285;
    (t19152, t19157, t19161, t19165, t19169)
}
