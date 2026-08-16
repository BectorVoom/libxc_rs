//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3854/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3854(t22212: f64, t2496: f64, t48280: f64, t48282: f64, t48285: f64, t1317: f64, t22193: f64, t39807: f64, t39813: f64, t47067: f64, t47070: f64, t47072: f64, t47076: f64, t73474: f64, t73477: f64, t73482: f64, t73494: f64, t73516: f64, t73517: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74106 = t22212 * t2496;
    let t74107 = 0.17315859105681463759e2_f64 * t74106;
    let t74108 = 0.11393789434848516923e-2_f64 * t48280;
    let t74109 = 0.70178683471615754484e1_f64 * t48282;
    let t74110 = 0.70178683471615754484e1_f64 * t48285;
    let t74111 = t1317 * t22193;
    let t74112 = 8.0_f64 * t74111;
    let t74113 = t73474 + t73477 + t39807 - t39813 - t73482 - t73494 + t73516 + t47067 - t73517 - t74107 + t47070 - t47072 - t74108 - t74109 - t47076 + t74110 + t74112;
    (t74107, t74108, t74109, t74110, t74112, t74113)
}
