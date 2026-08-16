//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1012/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1012(t10251: f64, t10254: f64, t10255: f64, t10256: f64, t10258: f64, t10260: f64, t10261: f64, t10262: f64, t4688: f64, t4711: f64, t4714: f64, t4718: f64, t4799: f64, t4803: f64, t4807: f64, t4815: f64, t8011: f64, t8022: f64) -> f64 {
    let t11312 = t10251 - t4799 - t4803 + t4807 + t10254 - t4815 + t4688 + t4711 - t4714 - t4718 - t8011 - t10255 - t10256 + t10258 - t10260 - t10261 - t10262 - t8022;
    t11312
}
