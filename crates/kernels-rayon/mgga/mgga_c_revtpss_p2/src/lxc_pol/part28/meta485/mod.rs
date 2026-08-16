//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta485 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1844;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1845;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta485(t14365: f64, t25759: f64, t1113: f64, t775: f64, t2430: f64, t33: f64, t2408: f64, t890: f64, t2832: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25436: f64, t25440: f64, t25445: f64, t25752: f64, t3351: f64, t4541: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64, t265: f64, t502: f64, t25743: f64, t2003: f64, t2258: f64, t57: f64, t606: f64, t7215: f64, t25751: f64, t4135: f64, t4147: f64, t2034: f64, t2014: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t116: f64, t6982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25791) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1844(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t1940, t1963, t2403, t25206, t25436, t25440, t25445, t25752, t3351, t4541, t7087, t7091, t7200, t7207);
        let (t25792, t25800, t25802, t25803, t25804) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1845(t33, t265, t502, t25743, t2003, t2258, t25791, t57, t606, t7215, t25751, t4135, t4147, t2034, t2014, dens_threshold, rho1, zeta_threshold);
        let t25805 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1846(t116, t6982);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25792, t25800, t25802, t25803, t25804, t25805)
}
