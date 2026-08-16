//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta224(t342: f64, t4980: f64, t3154: f64, t3302: f64, t1043: f64, t4893: f64, t1071: f64, t1089: f64, t1668: f64, t378: f64, t4866: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4981, t4982, t4983, t4984, t4988, t4992, t4995) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1036(t342, t4980, t3154, t3302, t1043, t4893, t1071, t1089, t1668, t378, t4866, t3316);
    (t4981, t4982, t4983, t4984, t4988, t4992, t4995)
}
