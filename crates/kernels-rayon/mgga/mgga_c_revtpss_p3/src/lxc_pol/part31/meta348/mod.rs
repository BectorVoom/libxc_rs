//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta348(t2482: f64, t4000: f64, t814: f64, t136: f64, t550: f64, t220: f64, t124: f64, t1882: f64, t5675: f64, t5609: f64, t9794: f64, t9793: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13846, t13847, t13848, t13850, t13851, t13857, t13858) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1361(t2482, t4000, t814, t136, t550, t220, t124, t1882, t5675, t5609, t9794, t9793);
    (t13846, t13847, t13848, t13850, t13851, t13857, t13858)
}
