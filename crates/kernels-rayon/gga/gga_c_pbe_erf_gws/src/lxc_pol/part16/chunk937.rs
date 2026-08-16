//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 937/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk937(t8230: f64, t8254: f64, t138: f64, t1572: f64, t1577: f64, t1578: f64, t1590: f64, t2902: f64, t2905: f64, t2919: f64, t514: f64, t520: f64, t5844: f64, t5847: f64, t5854: f64, t8204: f64, t8206: f64, t8209: f64, t8218: f64, t8221: f64, t8224: f64, t985: f64) -> f64 {
    let t8255 = t8230 + t8254;
    let t8257 = t138 * t8204 - 2.0_f64 * t1572 * t2919 + 4.0_f64 * t1577 * t8221 + 2.0_f64 * t1577 * t8224 + 2.0_f64 * t1578 * t8209 - t1590 * t2902 + 4.0_f64 * t2905 * t5847 - t514 * t8255 - 2.0_f64 * t520 * t8206 - t5844 * t985 - 6.0_f64 * t5854 * t8218;
    t8257
}
