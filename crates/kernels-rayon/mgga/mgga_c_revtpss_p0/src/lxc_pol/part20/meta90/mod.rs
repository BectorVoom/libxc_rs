//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk528;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk529;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk530;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta90(t190: f64, t2251: f64, t2611: f64, t606: f64, t750: f64, t706: f64, t186: f64, t215: f64, t685: f64, t755: f64, t72: f64, t752: f64, t757: f64, t2492: f64, t2596: f64, t745: f64, t760: f64, t123: f64, t192: f64, t676: f64, t762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2612, t2614, t2615, t2617, t2619) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk528(t190, t2251, t2611, t606, t750, t706, t186, t215, t685);
        let (t2621, t2622, t2624, t2626) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk529(t2619, t755, t72, t752, t757, t2492, t2596, t745);
        let (t2628, t2629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk530(t2626, t760, t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk531(t676, t762);
    (t2612, t2614, t2615, t2617, t2619, t2621, t2622, t2624, t2626, t2628, t2629, t2630)
}
