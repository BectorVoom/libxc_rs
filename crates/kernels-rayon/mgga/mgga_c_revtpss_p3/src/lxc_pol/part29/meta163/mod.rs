//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk798;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta163(t3497: f64, t3523: f64, t1161: f64, t1170: f64, t1180: f64, t1189: f64, t3378: f64, t3381: f64, t3388: f64, t3430: f64, t3438: f64, t3444: f64, t3447: f64, t3452: f64, t3454: f64, t3472: f64, t3477: f64, t3480: f64, t3489: f64, t3491: f64, t3496: f64, t3498: f64, t3516: f64, t3521: f64, t435: f64, t300: f64, t1175: f64) -> (f64, f64, f64, f64) {
        let (t3524, t3527) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk798(t3497, t3523, t1161, t1170, t1180, t1189, t3378, t3381, t3388, t3430, t3438, t3444, t3447, t3452, t3454, t3472, t3477, t3480, t3489, t3491, t3496, t3498, t3516, t3521, t435);
        let (t3528, t3530, t3531) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk799(t300, t3527, t3489, t1175);
    (t3524, t3528, t3530, t3531)
}
