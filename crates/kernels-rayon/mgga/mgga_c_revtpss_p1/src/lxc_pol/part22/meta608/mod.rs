//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2506;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2507;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta608(t3011: f64, t6205: f64, t4733: f64, t981: f64, t15258: f64, t4732: f64, t4719: f64, t4729: f64, t19136: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64, t19466: f64, t1089: f64, t378: f64, t3302: f64, t357: f64, t4866: f64, t4893: f64, t1071: f64, t6299: f64, t1043: f64, t16560: f64, t19450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19467, t19468, t19470, t19471, t19473, t19475, t19476) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2506(t3011, t6205, t4733, t981, t15258, t4732, t4719, t4729, t19136, t19143, t19145, t19149, t19152, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337);
        let t19477 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2507(t19466, t19476);
        let (t19479, t19482, t19484, t19488, t19492) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2508(t1089, t19477, t378, t3302, t357, t4866, t4893, t1071, t6299, t1043, t16560, t19450);
    (t19467, t19468, t19470, t19471, t19473, t19475, t19477, t19479, t19482, t19484, t19488, t19492)
}
