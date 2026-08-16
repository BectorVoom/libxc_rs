//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1082/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1082(t11762: f64, t11766: f64, t11768: f64, t11770: f64, t11772: f64, t11775: f64, t11780: f64, t11784: f64, t11789: f64, t11796: f64, t8969: f64, t8971: f64, t8973: f64) -> f64 {
    let t12153 = -t8969 + t8971 + t8973 - t11762 + t11766 - t11768 + t11770 + t11772 - t11775 - t11780 - t11784 + t11789 - t11796;
    t12153
}
