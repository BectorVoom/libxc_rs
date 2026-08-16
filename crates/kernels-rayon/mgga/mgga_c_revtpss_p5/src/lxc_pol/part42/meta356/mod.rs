//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1168;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta356(t1263: f64, t3362: f64, t3172: f64, t5298: f64, t3711: f64, t5278: f64, t5269: f64, t1261: f64, t12256: f64, t13099: f64, t1224: f64, t140: f64, t5052: f64, t1222: f64, t3636: f64, t5391: f64, t5381: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64, t480: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17202, t17211, t17219, t17227, t17235, t17240) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1168(t1263, t3362, t3172, t5298, t3711, t5278, t5269, t1261, t12256, t13099, t1224, t140);
        let (t17243, t17258, t17260, t17283, t17288, t17289, t17290) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1169(t17240, t5052, t1222, t3636, t5391, t5381, t1803, t3666, t1208, t5215, t225, t480);
    (t17202, t17211, t17219, t17227, t17235, t17243, t17258, t17260, t17283, t17288, t17289, t17290)
}
