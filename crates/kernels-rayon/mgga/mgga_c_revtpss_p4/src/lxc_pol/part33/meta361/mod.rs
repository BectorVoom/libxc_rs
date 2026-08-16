//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1386;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta361(t2398: f64, t4305: f64, t177: f64, t4392: f64, t762: f64, t2626: f64, t4398: f64, t10439: f64, t162: f64, t2516: f64, t2496: f64, t2619: f64, t4302: f64, t4186: f64, t750: f64, t706: f64, t4395: f64, t4537: f64, t892: f64, t123: f64, t1534: f64, t2630: f64, t1469: f64, t749: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14317, t14324, t14328, t14330, t14334, t14336, t14339) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1386(t2398, t4305, t177, t4392, t762, t2626, t4398, t10439, t162, t2516, t2496, t2619, t4302);
        let (t14343, t14345, t14353, t14363, t14370) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1387(t4186, t750, t706, t4395, t4537, t892, t123, t1534, t2630, t1469, t749, t606);
    (t14317, t14324, t14328, t14330, t14334, t14336, t14339, t14343, t14345, t14353, t14363, t14370)
}
