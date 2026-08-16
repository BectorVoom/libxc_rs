//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2404;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2405;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta670(t1041: f64, t1046: f64, t42994: f64, t3057: f64, t3316: f64, t4891: f64, t3298: f64, t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64, t11858: f64, t15688: f64, t12077: f64, t15905: f64, t994: f64, t11725: f64, t828: f64, t225: f64, t42059: f64, t1053: f64, t11940: f64, t11240: f64, t11628: f64, t42646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42996, t43044, t43050, t43066, t43069) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2404(t1041, t1046, t42994, t3057, t3316, t4891, t3298, t11670, t11772, t3114, t11773, t11926);
        let (t43082, t43105, t43131, t43154, t43161, t43207) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2405(t11858, t15688, t12077, t15905, t994, t11725, t828, t225, t42059, t1053, t11940, t11240, t11628, t42646);
    (t42996, t43044, t43050, t43066, t43069, t43082, t43105, t43131, t43154, t43161, t43207)
}
