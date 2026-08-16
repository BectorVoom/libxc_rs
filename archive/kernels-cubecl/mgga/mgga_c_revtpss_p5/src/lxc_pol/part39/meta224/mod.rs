//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk881;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk882;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk883;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk884;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk885;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta224<F: Float>(t1699: F, t3336: F, t1100: F, t1102: F, t198: F, t336: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F, t5019: F, t5023: F, t30: F, t265: F, t393: F, t4560: F, t1106: F, t1468: F, t1469: F, t1587: F, t1704: F, t395: F, t4186: F, t45: F, t4568: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1716: F, t689: F, t3362: F, t3360: F, t128: F, t3367: F, t1120: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5024, t5027) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk881::<F>(t1699, t3336, t1100, t1102, t198, t336, t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736, t5019, t5023);
        let (t5028, t5035) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk882::<F>(t30, t265, t393, t4560, t5027, t1106, t1468, t1469, t1587, t1704, t395, t4186, t45, t4568, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t5044 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk883::<F>(t1716, t689);
        let (t5046, t5047, t5048, t5049) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk884::<F>(t1469, t3362, t606, t3360, t128);
        let (t5051, t5052, t5053, t5054) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk885::<F>(t1469, t3367, t606, t1120, t128);
    (t5024, t5028, t5035, t5044, t5046, t5047, t5048, t5049, t5051, t5052, t5053, t5054)
}
