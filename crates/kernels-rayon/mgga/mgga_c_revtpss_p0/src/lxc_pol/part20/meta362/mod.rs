//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1315;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1316;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1317;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta362(t268: f64, t675: f64, t9273: f64, t9276: f64, t192: f64, t9450: f64, t9501: f64, t2258: f64, t2609: f64, t706: f64, t9476: f64, t9508: f64, t2582: f64, t2584: f64, t39480: f64, t10587: f64, t2516: f64, t157: f64, t190: f64, t39443: f64, t2401: f64, t2519: f64, t9306: f64, t9518: f64, t9540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39760 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1315(t268, t675, t9273, t9276);
        let (t39762, t39764, t39767, t39768, t39770, t39773) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1316(t192, t268, t9450, t9501, t2258, t2609, t706, t9476, t9508, t2582, t2584, t39480);
        let (t39775, t39778, t39780, t39783) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1317(t10587, t2516, t157, t190, t39443, t2401, t2609, t2519, t268, t9306);
        let t39786 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1318(t268, t9518, t9540);
    (t39760, t39762, t39764, t39767, t39768, t39770, t39773, t39775, t39778, t39780, t39783, t39786)
}
