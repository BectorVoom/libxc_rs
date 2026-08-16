//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta477(t14365: f64, t25759: f64, t1113: f64, t775: f64, t2430: f64, t33: f64, t2408: f64, t890: f64, t2832: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25436: f64, t25440: f64, t25445: f64, t25752: f64, t3351: f64, t4541: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25760, t25763, t25767, t25778, t25781, t25784, t25791) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1803(t14365, t25759, t1113, t775, t2430, t33, t2408, t890, t2832, t1940, t1963, t2403, t25206, t25436, t25440, t25445, t25752, t3351, t4541, t7087, t7091, t7200, t7207);
    (t25760, t25763, t25767, t25778, t25781, t25784, t25791)
}
