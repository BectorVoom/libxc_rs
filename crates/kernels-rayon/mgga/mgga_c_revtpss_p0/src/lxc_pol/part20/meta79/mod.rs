//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk485;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk486;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk487;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta79(t2398: f64, t707: f64, t150: f64, t2389: f64, t190: f64, t198: f64, t206: f64, t890: f64, t892: f64, t45: f64, t57: f64, t261: f64, t2258: f64, t706: f64, t2251: f64, t766: f64, t80: f64, t770: f64, t83: f64, zeta_threshold: f64, t125: f64, t215: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2400, t2401, t2402, t2403) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk485(t2398, t707, t150, t2389, t190, t198, t206);
        let (t2404, t2408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk486(t890, t892);
        let (t2410, t2411, t2414, t2416, t2430) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk487(t45, t57, t261, t190, t2258, t706, t2251, t766, t80, t770, t83, zeta_threshold);
        let t2434 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk488(t125, t215);
    (t2400, t2401, t2402, t2403, t2404, t2408, t2410, t2411, t2414, t2416, t2430, t2434)
}
