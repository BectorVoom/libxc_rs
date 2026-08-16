//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1563;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta528(t12866: f64, t58895: f64, t6639: f64, t17448: f64, t21090: f64, t12916: f64, t24730: f64, t5340: f64, t12809: f64, t24839: f64, t21063: f64, t5362: f64, t17308: f64, t20846: f64, t24639: f64, t3172: f64, t3711: f64, t13062: f64, t24545: f64, t1261: f64, t24807: f64, t17377: f64, t20786: f64, t24604: f64, t5384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83758, t83783, t83798, t83812, t83849) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1562(t12866, t58895, t6639, t17448, t21090, t12916, t24730, t5340, t12809, t24839, t21063, t5362);
        let (t83851, t83860, t83863, t83871, t83891, t83897) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1563(t17308, t20846, t24639, t3172, t3711, t13062, t24545, t1261, t24807, t17377, t20786, t24604, t5384);
    (t83758, t83783, t83798, t83812, t83849, t83851, t83860, t83863, t83871, t83891, t83897)
}
