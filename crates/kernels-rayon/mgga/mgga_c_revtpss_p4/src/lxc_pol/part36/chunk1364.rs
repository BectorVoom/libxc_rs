//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1364/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1364(t104529: f64, t105365: f64, t105530: f64, t112706: f64, t112902: f64, t1287: f64, t1769: f64, t1775: f64, t1794: f64, t1829: f64, t2142: f64, t2144: f64, t24697: f64, t24698: f64, t29129: f64, t29275: f64, t30739: f64, t30744: f64, t30767: f64, t30850: f64, t30854: f64, t30874: f64, t30878: f64, t30886: f64, t30907: f64, t6564: f64, t6744: f64, t7636: f64, t7637: f64, t7651: f64, t7652: f64, t8190: f64, t8192: f64, t97041: f64, t97348: f64) -> f64 {
    let t116607 = 0.65854491829355115987e0_f64 * t24698 * t2144 + 0.19756347548806534796e1_f64 * t6564 * t8192 - 0.78062653693846795158e1_f64 * t97041 * t30739 * t1794 * t1287 + 0.26020884564615598386e1_f64 * t7651 * t7652 * t8190 * t6744 + 0.10408353825846239354e2_f64 * t7636 * t7652 * t30886 * t1769 - 0.52041769129231196772e1_f64 * t105530 * t30854 - 0.19756347548806534796e1_f64 * t112706 * t1829 + 0.52041769129231196772e1_f64 * t105365 * t30850 - 0.8673628188205199462e0_f64 * t7636 * t7637 * t2142 * t24697 - 0.13010442282307799193e1_f64 * t29129 * t30874 - 0.26020884564615598386e1_f64 * t104529 * t30878 - 0.39512695097613069591e1_f64 * t112902 * t1775 - 0.52041769129231196772e1_f64 * t29275 * t30744 + 0.10408353825846239354e2_f64 * t29275 * t30907 - 0.78062653693846795158e1_f64 * t97348 * t30767 * t1794 * t1287;
    t116607
}
