//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta338(t3566: f64, t3754: f64, t1269: f64, t1284: f64, t1209: f64, t1204: f64, t3781: f64, t5462: f64, t5477: f64, t3634: f64, t828: f64, t3624: f64, t3746: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12717, t12723, t12744, t12751, t12756, t12772, t12784) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1349(t3566, t3754, t1269, t1284, t1209, t1204, t3781, t5462, t5477, t3634, t828, t3624, t3746);
    (t12717, t12723, t12744, t12751, t12756, t12772, t12784)
}
