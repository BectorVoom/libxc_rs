//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta487(t3167: f64, t7120: f64, t1033: f64, t3173: f64, t7122: f64, t1007: f64, t7106: f64, t1968: f64, t3080: f64, t7105: f64, t800: f64, t3244: f64, t7111: f64, t3111: f64, t7132: f64, t1058: f64, t7126: f64, t1973: f64, t3201: f64, t7114: f64, t1020: f64, t7131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25525, t25526, t25529, t25535, t25538, t25539) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1780(t3167, t7120, t1033, t3173, t7122, t1007, t7106, t1968, t3080, t7105, t800);
        let (t25543, t25551, t25557, t25560, t25564, t25569) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1781(t3244, t7111, t3111, t7132, t1058, t7126, t1973, t3201, t7114, t1020, t7131);
    (t25525, t25526, t25529, t25535, t25538, t25539, t25543, t25551, t25557, t25560, t25564, t25569)
}
