//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2044;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta580(t3268: f64, t7143: f64, t3057: f64, t25460: f64, t25698: f64, t1035: f64, t25586: f64, t93484: f64, t994: f64, t1071: f64, t7150: f64, t8521: f64, t359: f64, t42066: f64, t3143: f64, t36870: f64, t1983: f64, t1981: f64, t42058: f64, t1982: f64, t11120: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93920, t93921, t93928, t93939, t93959, t93963) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2044(t3268, t7143, t3057, t25460, t25698, t1035, t25586, t93484, t994, t1071, t7150, t8521);
        let (t93968, t93983, t93994, t94005, t94014) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2045(t359, t42066, t3143, t36870, t1983, t1981, t42058, t7143, t1982, t93484, t11120, t3140);
    (t93920, t93921, t93928, t93939, t93959, t93963, t93968, t93983, t93994, t94005, t94014)
}
