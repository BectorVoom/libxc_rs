//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1844;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1845;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta485<F: Float>(t14365: F, t25759: F, t1113: F, t775: F, t2430: F, t33: F, t2408: F, t890: F, t2832: F, t1940: F, t1963: F, t2403: F, t25206: F, t25436: F, t25440: F, t25445: F, t25752: F, t3351: F, t4541: F, t7087: F, t7091: F, t7200: F, t7207: F, t265: F, t502: F, t25743: F, t2003: F, t2258: F, t57: F, t606: F, t7215: F, t25751: F, t4135: F, t4147: F, t2034: F, t2014: F, dens_threshold: F, rho1: F, zeta_threshold: F, t116: F, t6982: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25791) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1844::<F>(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t1940, t1963, t2403, t25206, t25436, t25440, t25445, t25752, t3351, t4541, t7087, t7091, t7200, t7207);
        let (t25792, t25800, t25802, t25803, t25804) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1845::<F>(t33, t265, t502, t25743, t2003, t2258, t25791, t57, t606, t7215, t25751, t4135, t4147, t2034, t2014, dens_threshold, rho1, zeta_threshold);
        let t25805 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1846::<F>(t116, t6982);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25792, t25800, t25802, t25803, t25804, t25805)
}
