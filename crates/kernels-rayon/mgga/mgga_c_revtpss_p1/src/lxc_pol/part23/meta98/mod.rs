//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta98 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk662;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk663;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk664;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk665;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta98(t177: f64, t752: f64, t762: f64, t717: f64, t750: f64, t675: f64, t723: f64, t169: f64, t722: f64, t164: f64, t729: f64, t730: f64, t2435: f64, t2439: f64, t2502: f64, t2504: f64, t2509: f64, t2511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2523 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk662(t177, t752);
        let (t2524, t2526, t2531, t2535, t2536, t2537, t2538) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk663(t2523, t762, t717, t750, t675, t723, t169, t722, t164, t729);
        let (t2539, t2548) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk664(t2538, t730, t2435, t2439, t2502, t2504, t2509, t2511);
        let (t2549, t2552) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk665(t2548, t730, t722);
    (t2523, t2524, t2526, t2531, t2535, t2536, t2537, t2538, t2539, t2548, t2549, t2552)
}
