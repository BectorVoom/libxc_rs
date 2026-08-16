//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta768 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta768(t3599: f64, t56802: f64, t3609: f64, t3623: f64, t53739: f64, t13127: f64, t1214: f64, t3611: f64, t12831: f64, t17395: f64, t13148: f64, t17728: f64, t460: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56803, t56806, t56878, t56879, t56947, t56953, t56997, t57005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2568(t3599, t56802, t3609, t3623, t53739, t13127, t1214, t3611, t12831, t17395, t13148, t17728, t460, t489);
    (t56803, t56806, t56878, t56879, t56947, t56953, t56997, t57005)
}
