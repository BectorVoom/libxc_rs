//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta416 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1361;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta416(t43043: f64, t4891: f64, t3057: f64, t3298: f64, t11773: f64, t11926: f64, t11858: f64, t15688: f64, t12077: f64, t15905: f64, t994: f64, t11725: f64, t828: f64, t225: f64, t42059: f64, t366: f64, t2857: f64, t3154: f64, t271: f64, t2852: f64, t41296: f64, t11986: f64, t11631: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43044, t43050, t43069, t43082, t43105, t43131) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1361(t43043, t4891, t3057, t3298, t11773, t11926, t11858, t15688, t12077, t15905, t994, t11725, t828);
        let (t43154, t43155, t43174, t43223, t43240, t43253) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1362(t225, t42059, t366, t2857, t3154, t271, t2852, t41296, t11986, t828, t11631, t905);
    (t43044, t43050, t43069, t43082, t43105, t43131, t43154, t43155, t43174, t43223, t43240, t43253)
}
