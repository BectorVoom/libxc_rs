//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2462;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta665(t3057: f64, t3316: f64, t4891: f64, t3298: f64, t3059: f64, t3154: f64, t1045: f64, t2853: f64, t999: f64, t11774: f64, t127: f64, t3096: f64, t3128: f64, t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64, t11858: f64, t15688: f64, t1020: f64, t12003: f64, t12077: f64, t15905: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43044, t43050, t43051, t43057, t43063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2462(t3057, t3316, t4891, t3298, t3059, t3154, t1045, t2853, t999, t11774, t127, t3096, t3128);
        let (t43065, t43066, t43069, t43082, t43091, t43105) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2463(t11670, t11772, t3114, t11773, t11926, t11858, t15688, t1020, t12003, t12077, t15905, t994);
    (t43044, t43050, t43051, t43057, t43063, t43065, t43066, t43069, t43082, t43091, t43105)
}
