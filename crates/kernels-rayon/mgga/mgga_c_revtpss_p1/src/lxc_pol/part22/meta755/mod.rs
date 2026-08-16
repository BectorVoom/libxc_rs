//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2831;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta755(t3105: f64, t3223: f64, t1041: f64, t11262: f64, t3135: f64, t12166: f64, t15905: f64, t994: f64, t11631: f64, t999: f64, t3046: f64, t3298: f64, t4891: f64, t11263: f64, t3169: f64, t3043: f64, t3140: f64, t3149: f64, t3160: f64, t11874: f64, t16048: f64, t12046: f64, t3114: f64, t42416: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42571, t42580, t42621, t42622, t42643) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2831(t3105, t3223, t1041, t11262, t3135, t12166, t15905, t994, t11631, t999, t3046, t3298, t4891);
        let (t42656, t42665, t42672, t42675, t42690, t42695) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832(t11263, t3169, t3043, t3140, t3149, t3160, t11874, t16048, t12046, t15905, t994, t3114, t42416);
    (t42571, t42580, t42621, t42622, t42643, t42656, t42665, t42672, t42675, t42690, t42695)
}
