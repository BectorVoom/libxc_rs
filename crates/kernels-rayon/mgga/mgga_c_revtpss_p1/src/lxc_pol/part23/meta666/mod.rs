//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2397;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta666(t1021: f64, t11970: f64, t11874: f64, t15688: f64, t11853: f64, t828: f64, t3181: f64, t675: f64, t283: f64, t2852: f64, t11144: f64, t3252: f64, t11852: f64, t126: f64, t12166: f64, t15905: f64, t994: f64, t11631: f64, t999: f64, t3046: f64, t3298: f64, t4891: f64, t1052: f64, t11243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42326, t42328, t42410, t42447, t42471, t42518) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2397(t1021, t11970, t11874, t15688, t11853, t828, t3181, t675, t283, t2852, t11144, t3252);
        let (t42534, t42621, t42622, t42643, t42646) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2398(t11852, t126, t12166, t15905, t994, t11631, t999, t3046, t3298, t4891, t1052, t11243);
    (t42326, t42328, t42410, t42447, t42471, t42518, t42534, t42621, t42622, t42643, t42646)
}
