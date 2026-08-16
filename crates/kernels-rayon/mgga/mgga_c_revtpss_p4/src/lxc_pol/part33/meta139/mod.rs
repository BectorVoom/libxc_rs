//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk747;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk748;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta139(t3655: f64, t481: f64, t1231: f64, t1256: f64, t225: f64, t3555: f64, t480: f64, t3566: f64, t1236: f64, t127: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3657, t3658, t3666) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk747(t3655, t481, t1231, t1256, t225, t3555);
        let (t3667, t3670) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk748(t3666, t480, t225, t3566);
        let (t3671, t3678) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk749(t3670, t480, t1236, t127, t371);
    (t3657, t3658, t3666, t3667, t3670, t3671, t3678)
}
