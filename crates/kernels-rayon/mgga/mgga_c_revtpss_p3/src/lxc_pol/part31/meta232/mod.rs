//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1036;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta232(t300: f64, t6212: f64, t6185: f64, t1642: f64, t4719: f64, t2986: f64, t6189: f64, t973: f64, t981: f64, t6205: f64, t964: f64, t3011: f64, t3014: f64, t3037: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1036(t300, t6212, t6185, t1642, t4719, t2986, t6189, t973, t981, t6205, t964, t3011);
        let (t6227, t6229, t6234, t6235) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1037(t3014, t6226, t981, t3037, t4571, t6094, t6098, t6102, t341);
    (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226, t6227, t6229, t6234, t6235)
}
