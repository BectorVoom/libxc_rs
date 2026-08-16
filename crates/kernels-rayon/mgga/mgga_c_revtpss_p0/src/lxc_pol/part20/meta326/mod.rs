//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1241;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta326(t10356: f64, t13020: f64, t1012: f64, t3367: f64, t404: f64, t12256: f64, t1204: f64, t3140: f64, t3599: f64, t11239: f64, t460: f64, t1242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13021, t13022, t13026, t13028, t13029, t13032, t13033, t13036, t13037) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1241(t10356, t13020, t1012, t3367, t404, t12256, t1204, t3140, t3599, t11239, t460, t1242);
    (t13021, t13022, t13026, t13028, t13029, t13032, t13033, t13036, t13037)
}
