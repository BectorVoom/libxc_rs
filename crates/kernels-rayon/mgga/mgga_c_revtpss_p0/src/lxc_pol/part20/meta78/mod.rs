//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta78 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk482;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk483;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk484;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta78(t45: f64, t57: f64, t2371: f64, t508: f64, t200: f64, t2251: f64, t2258: f64, t78: f64, t202: f64, t81: f64, t162: f64, t187: f64, t205: f64, t262: f64, zeta_threshold: f64, t775: f64, t705: f64, t716: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2372, t2375, t2382, t2389, t2390, t2392, t2393) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk482(t45, t57, t2371, t508, t200, t2251, t2258, t78, t202, t81, t162, t187, t205, t262, zeta_threshold);
        let t2394 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk483(t775);
        let t2398 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk484(t705, t716);
    (t2372, t2375, t2382, t2389, t2390, t2392, t2393, t2394, t2398)
}
