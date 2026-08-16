//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2182;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta575(t45: f64, t57: f64, t14441: f64, t10446: f64, t22671: f64, t22688: f64, t4377: f64, t5825: f64, t78: f64, t10457: f64, t4384: f64, t81: f64, t162: f64, t187: f64, zeta_threshold: f64, t150: f64, t190: f64, t1469: f64, t18305: f64, t4401: f64, t14613: f64, t6002: f64, t706: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t9542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23193, t23210, t23211, t23213) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2182(t45, t57, t14441, t10446, t22671, t22688, t4377, t5825, t78, t10457, t4384, t81, t162, t187, zeta_threshold);
        let (t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2183(t150, t23210, t190, t1469, t18305, t4401, t14613, t6002, t22671, t706, t10592, t10596, t10604, t10611, t23193, t23213, t9542);
    (t23193, t23210, t23211, t23213, t23214, t23215, t23216, t23218, t23220, t23221, t23223, t23224)
}
