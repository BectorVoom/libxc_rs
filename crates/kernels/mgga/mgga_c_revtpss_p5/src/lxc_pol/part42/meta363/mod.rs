//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1180;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta363<F: Float>(t1261: F, t17720: F, t1209: F, t489: F, t3623: F, t370: F, t3566: F, t1121: F, t1774: F, t13142: F, t17708: F, t13127: F, t1260: F, t5261: F, t3647: F, t5378: F, t247: F, t3634: F, t5056: F, t12916: F, t5334: F, t5331: F, t1778: F, t3682: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17721, t17729, t17736, t17737, t17747, t17753) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1180::<F>(t1261, t17720, t1209, t489, t3623, t370, t3566, t1121, t1774, t13142, t17708, t13127);
        let (t17763, t17767, t17771, t17791, t17792) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1181::<F>(t1260, t5261, t3647, t5378, t247, t3634, t5056, t1261, t12916, t5334, t5331, t1778, t3682);
    (t17721, t17729, t17736, t17737, t17747, t17753, t17763, t17767, t17771, t17791, t17792)
}
