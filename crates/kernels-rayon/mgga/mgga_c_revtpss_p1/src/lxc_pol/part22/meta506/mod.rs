//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta506(t3498: f64, t5205: f64, t1196: f64, t12485: f64, t1756: f64, t3524: f64, t3531: f64, t5198: f64, t12361: f64, t5068: f64, t12243: f64, t5109: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16639, t16641, t16643, t16645, t16647, t16649, t16651) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2249(t3498, t5205, t1196, t12485, t1756, t3524, t3531, t5198, t12361, t5068, t12243, t5109);
    (t16639, t16641, t16643, t16645, t16647, t16649, t16651)
}
