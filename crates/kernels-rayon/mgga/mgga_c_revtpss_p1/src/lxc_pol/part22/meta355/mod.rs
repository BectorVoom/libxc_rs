//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1855;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1856;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta355(t3106: f64, t3111: f64, t3156: f64, t3172: f64, t3150: f64, t11997: f64, t3144: f64, t3141: f64, t1032: f64, t3043: f64, t1040: f64, t1035: f64, t11239: f64, t342: f64, t3145: f64, t334: f64, t3259: f64, t359: f64, t3143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12007, t12009, t12010, t12012, t12013, t12021, t12046) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1855(t3106, t3111, t3156, t3172, t3150, t11997, t3144, t3141, t1032, t3043, t1040, t1035, t11239);
        let (t12047, t12050) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1856(t12046, t342, t3145, t334);
        let (t12073, t12077) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1857(t3259, t359, t11239, t3143);
    (t12007, t12009, t12010, t12012, t12013, t12021, t12046, t12047, t12050, t12073, t12077)
}
