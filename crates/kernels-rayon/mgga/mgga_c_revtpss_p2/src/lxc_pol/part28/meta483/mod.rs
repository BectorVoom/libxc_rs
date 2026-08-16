//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1834;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1835;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta483(t1071: f64, t3140: f64, t1078: f64, t1982: f64, t7135: f64, t988: f64, t7145: f64, t1976: f64, t3057: f64, t989: f64, t225: f64, t25586: f64, t385: f64, t11239: f64, t378: f64, t3143: f64, t3151: f64, t3304: f64, t3318: f64, t7168: f64, t1035: f64, t1043: f64, t1089: f64, t3133: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25640, t25648, t25651) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1834(t1071, t3140, t1078, t1982, t7135, t988, t7145, t1976, t3057);
        let (t25658, t25662, t25671, t25674) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1835(t1976, t989, t225, t25586, t385, t11239, t378, t1078, t1982, t3143, t3151, t3304);
        let (t25678, t25681, t25683, t25687, t25692) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1836(t3151, t3318, t7168, t1035, t7135, t1043, t1089, t3133, t1976, t3046);
    (t25640, t25648, t25651, t25658, t25662, t25671, t25674, t25678, t25681, t25683, t25687, t25692)
}
