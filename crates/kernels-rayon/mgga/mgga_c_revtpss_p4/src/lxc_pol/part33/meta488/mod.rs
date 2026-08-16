//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta488(t112: f64, t239: f64, t624: f64, t655: f64, t665: f64, t2339: f64, t68: f64, t2033: f64, t530: f64, t555: f64, t7063: f64, t1032: f64, t4075: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25822, t25823, t25824, t25826, t25864, t25875) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1780(t112, t239, t624, t655, t665, t2339, t68, t2033, t530, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1781(t1032, t4075, t545);
        let t25878 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1782(t25875, t25877);
    (t25822, t25823, t25824, t25826, t25864, t25875, t25876, t25877, t25878)
}
