//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk675;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk676;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk677;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk678;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta96(t45: f64, t2251: f64, t2258: f64, t2375: f64, t78: f64, t202: f64, zeta_threshold: f64, t57: f64, t81: f64, t162: f64, t187: f64, t205: f64, t262: f64, t775: f64, t705: f64, t716: f64, t707: f64, t150: f64, t190: f64, t198: f64, t206: f64, t890: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2381, t2382) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk675(t45, t2251, t2258, t2375, t78, t202, zeta_threshold);
        let (t2389, t2390, t2392, t2393, t2394) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk676(t57, t2251, t2258, t2382, t81, t2381, t162, t187, t205, t262, t775, zeta_threshold);
        let t2398 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk677(t705, t716);
        let (t2400, t2401, t2402, t2403) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk678(t2398, t707, t150, t2389, t190, t198, t206);
        let t2404 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk679(t890, t892);
    (t2382, t2389, t2390, t2392, t2393, t2394, t2398, t2400, t2401, t2402, t2403, t2404)
}
