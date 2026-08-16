//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 999/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk999(t10877: f64, t10880: f64, t10888: f64, t10890: f64, t10894: f64, t10895: f64, t10897: f64, t10901: f64, t10903: f64, t10904: f64, t10907: f64, t10912: f64, t10915: f64, t7753: f64, t7757: f64, t7775: f64) -> f64 {
    let t11221 = -t7753 + t7757 + t10877 + t10880 + t10888 + t10890 + t10894 + t7775 - t10895 - t10897 + t10901 + t10903 - t10904 + t10907 - t10912 - t10915;
    t11221
}
