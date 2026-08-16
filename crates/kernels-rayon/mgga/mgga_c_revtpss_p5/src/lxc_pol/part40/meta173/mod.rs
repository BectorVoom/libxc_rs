//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk764;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk765;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta173(t1353: f64, t30: f64, t33: f64, t525: f64, t605: f64, t2257: f64, t513: f64, t527: f64, t1113: f64, t3351: f64, t516: f64, t162: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t3829 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk764(t1353);
        let (t3833, t3834, t3841, t3842, t3850) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk765(t30, t33, t525, t605, t2257, t513, t527, t1113, t3351, t516, t162, zeta_threshold);
    (t3829, t3833, t3834, t3841, t3842, t3850)
}
