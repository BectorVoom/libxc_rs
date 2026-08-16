//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1883;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta453(t1089: f64, t19477: f64, t378: f64, t3302: f64, t357: f64, t4866: f64, t4893: f64, t1071: f64, t6299: f64, t1043: f64, t16560: f64, t19450: f64, t6258: f64, t3153: f64, t6305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19479, t19482, t19483, t19484, t19488, t19491, t19492) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1883(t1089, t19477, t378, t3302, t357, t4866, t4893, t1071, t6299, t1043, t16560, t19450);
        let (t19497, t19498, t19501) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1884(t1043, t6258, t1089, t3153, t6305);
    (t19479, t19482, t19483, t19484, t19488, t19491, t19492, t19497, t19498, t19501)
}
