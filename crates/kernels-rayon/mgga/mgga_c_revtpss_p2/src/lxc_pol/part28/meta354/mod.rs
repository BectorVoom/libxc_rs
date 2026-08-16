//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta354(t12009: f64, t3150: f64, t1032: f64, t3043: f64, t1040: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t12010, t12020, t12021, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1374(t12009, t3150, t1032, t3043, t1040, t1035, t11239, t342, t3145, t334);
    (t12010, t12020, t12021, t12046, t12047, t12050)
}
