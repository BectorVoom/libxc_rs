//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 933/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk933(t127: f64, t1533: f64, t2893: f64, t481: f64, t5784: f64, t5788: f64, t5806: f64, t8155: f64, t8158: f64, t8160: f64, t8162: f64, t8171: f64, t8174: f64, t8177: f64, t8202: f64) -> f64 {
    let t8204 = t8155 + t8158 - 0.48968e0_f64 * t8160 + 0.1175232e2_f64 * t127 * t8162 * t481 + 0.587616e1_f64 * t127 * t2893 * t1533 + t8171 + t8174 - 4.0_f64 / 9.0_f64 * t5784 + t5788 / 6.0_f64 + t8177 - 0.293808e1_f64 * t5806 + t8202;
    t8204
}
