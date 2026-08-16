//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta103 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk615;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk616;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta103(t2496: f64, t760: f64, t128: f64, t131: f64, t136: f64, t2457: f64, t2470: f64, t684: f64, t692: f64, t2435: f64, t2439: f64, t738: f64, t745: f64, t675: f64, t681: f64, t268: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2498, t2501, t2502, t2504, t2508, t2509, t2511, t2514) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk615(t2496, t760, t128, t131, t136, t2457, t2470, t684, t692, t2435, t2439);
        let t2516 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk616(t2514, t738, t745);
        let (t2518, t2519, t2522) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk617(t2516, t760, t675, t681, t268, t702);
    (t2498, t2501, t2502, t2504, t2508, t2509, t2511, t2514, t2516, t2518, t2519, t2522)
}
