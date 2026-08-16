//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1359;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta393(t670: f64, t6765: f64, t1843: f64, t4292: f64, t1310: f64, t5920: f64, t116: f64, t5876: f64, t4343: f64, t4542: f64, t2404: f64, t5966: f64, t14613: f64, t162: f64, t4403: f64, t14312: f64, t5940: f64, t705: f64, t707: f64, t10605: f64, t6002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18232, t18235, t18242, t18245) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1359(t670, t6765, t1843, t4292, t1310, t5920, t116, t5876);
        let (t18253, t18256, t18261, t18262, t18265, t18267) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1360(t4343, t4542, t2404, t5966, t14613, t162, t4403, t14312, t5940, t705, t707, t10605, t6002);
    (t18232, t18235, t18242, t18245, t18253, t18256, t18261, t18262, t18265, t18267)
}
