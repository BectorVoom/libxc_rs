//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1358;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta344(t11132: f64, t2942: f64, t941: f64, t2986: f64, t960: f64, t2979: f64, t300: f64, t1034: f64, t3154: f64, t357: f64, t3129: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11534, t11548, t11554, t11560, t11574, t11591, t11627, t11631, t11643) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1358(t11132, t2942, t941, t2986, t960, t2979, t300, t1034, t3154, t357, t3129, t3172);
    (t11534, t11548, t11554, t11560, t11574, t11591, t11627, t11631, t11643)
}
