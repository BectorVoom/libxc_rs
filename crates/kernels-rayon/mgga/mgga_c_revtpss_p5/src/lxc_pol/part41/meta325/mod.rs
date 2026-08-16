//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1111;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta325(t2626: f64, t4398: f64, t10439: f64, t162: f64, t2516: f64, t2496: f64, t2619: f64, t4302: f64, t4186: f64, t750: f64, t706: f64, t4395: f64, t4537: f64, t892: f64, t123: f64, t1534: f64, t2630: f64, t1469: f64, t749: f64, t606: f64, t4401: f64, t4391: f64, t705: f64, t2615: f64, t4311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14328, t14330, t14334, t14336, t14339, t14343, t14345) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1111(t2626, t4398, t10439, t162, t2516, t2496, t2619, t4302, t4186, t750, t706, t4395);
        let (t14353, t14363, t14372, t14386, t14433) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1112(t4537, t892, t123, t1534, t2630, t1469, t749, t606, t4401, t4391, t705, t2615, t4311);
    (t14328, t14330, t14334, t14336, t14339, t14343, t14345, t14353, t14363, t14372, t14386, t14433)
}
