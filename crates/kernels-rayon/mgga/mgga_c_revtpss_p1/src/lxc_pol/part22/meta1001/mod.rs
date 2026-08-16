//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1001 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1001(t4707: f64, t3011: f64, t3014: f64, t981: f64, t11108: f64, t6396: f64, t2874: f64, t63657: f64, t935: f64, t19471: f64, t3022: f64, t15534: f64, t4719: f64, t19133: f64, t2989: f64, t15559: f64, t11591: f64, t6223: f64, t19049: f64, t3026: f64, t15556: f64, t19146: f64, t3007: f64, t16612: f64, t19137: f64, t3329: f64, t3333: f64, t5023: f64, t5024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63902, t63906, t63907, t63916, t63918, t63920) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3406(t4707, t3011, t3014, t981, t11108, t6396, t2874, t63657, t935, t19471, t3022, t15534, t4719);
        let (t63923, t63925, t63927, t63929, t63934, t63937) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3407(t19133, t2989, t981, t15559, t4719, t11591, t6223, t19049, t3026, t15556, t19146, t3007);
        let t63938 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3408(t16612, t19137, t3329, t3333, t5023, t5024, t63906, t63907, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937);
    (t63902, t63906, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937, t63938)
}
