//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta325(t221: f64, t346: f64, t68: f64, t345: f64, t245: f64, t3089: f64, t3088: f64, t3114: f64, t11223: f64, t225: f64, t366: f64, t1026: f64, t371: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11735, t11737, t11772, t11773, t11774, t11788, t11789, t11817) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1328(t221, t346, t68, t345, t245, t3089, t3088, t3114, t11223, t225, t366, t1026, t371, t676);
    (t11735, t11737, t11772, t11773, t11774, t11788, t11789, t11817)
}
