//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta671(t271: f64, t2852: f64, t1054: f64, t11970: f64, t11986: f64, t828: f64, t11631: f64, t905: f64, t606: f64, t1086: f64, t11223: f64, t3090: f64, t11200: f64, t11671: f64, t11926: f64, t16565: f64, t994: f64, t42859: f64, t42862: f64, t342: f64, t3145: f64, t368: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43222, t43238, t43240, t43254, t43285) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2406(t271, t2852, t1054, t11970, t11986, t828, t11631, t905, t606, t1086, t11223, t3090);
        let (t43291, t43297, t43341, t43346, t43347, t43350) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2407(t1086, t11200, t3090, t11671, t11926, t16565, t994, t42859, t42862, t342, t3145, t368);
    (t43222, t43238, t43240, t43254, t43285, t43291, t43297, t43341, t43346, t43347, t43350)
}
