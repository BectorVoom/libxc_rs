//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 992/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk992(t10400: f64, t10405: f64, t10410: f64, t10413: f64, t10417: f64, t10421: f64, t10423: f64, t10428: f64, t10432: f64, t10436: f64, t10441: f64, t10446: f64, t5919: f64, t5922: f64, t7190: f64, t7193: f64, t8425: f64) -> f64 {
    let t11202 = -t10400 + t10405 + t10410 - t10413 + t10417 + t10421 + t10423 + 0.22363485482220676312e-1_f64 * t8425 - t5919 + t5922 - t7190 + t7193 + t10428 + t10432 - t10436 - t10441 + t10446;
    t11202
}
