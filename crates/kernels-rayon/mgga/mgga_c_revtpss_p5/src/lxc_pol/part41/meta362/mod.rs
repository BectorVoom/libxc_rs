//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1178;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta362(t1261: f64, t17720: f64, t1209: f64, t489: f64, t3623: f64, t370: f64, t3566: f64, t1121: f64, t1774: f64, t13142: f64, t17708: f64, t13127: f64, t1260: f64, t5261: f64, t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t12916: f64, t5334: f64, t5331: f64, t1778: f64, t3682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17721, t17729, t17736, t17737, t17747, t17753) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1178(t1261, t17720, t1209, t489, t3623, t370, t3566, t1121, t1774, t13142, t17708, t13127);
        let (t17763, t17767, t17771, t17791, t17792) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1179(t1260, t5261, t3647, t5378, t247, t3634, t5056, t1261, t12916, t5334, t5331, t1778, t3682);
    (t17721, t17729, t17736, t17737, t17747, t17753, t17763, t17767, t17771, t17791, t17792)
}
