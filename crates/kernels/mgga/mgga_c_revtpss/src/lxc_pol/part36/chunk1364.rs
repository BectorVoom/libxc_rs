//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1364/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1364<F: Float>(t104529: F, t105365: F, t105530: F, t112706: F, t112902: F, t1287: F, t1769: F, t1775: F, t1794: F, t1829: F, t2142: F, t2144: F, t24697: F, t24698: F, t29129: F, t29275: F, t30739: F, t30744: F, t30767: F, t30850: F, t30854: F, t30874: F, t30878: F, t30886: F, t30907: F, t6564: F, t6744: F, t7636: F, t7637: F, t7651: F, t7652: F, t8190: F, t8192: F, t97041: F, t97348: F) -> F {
    let t116607 = F::cast_from(0.65854491829355115987e0_f64) * t24698 * t2144 + F::cast_from(0.19756347548806534796e1_f64) * t6564 * t8192 - F::cast_from(0.78062653693846795158e1_f64) * t97041 * t30739 * t1794 * t1287 + F::cast_from(0.26020884564615598386e1_f64) * t7651 * t7652 * t8190 * t6744 + F::cast_from(0.10408353825846239354e2_f64) * t7636 * t7652 * t30886 * t1769 - F::cast_from(0.52041769129231196772e1_f64) * t105530 * t30854 - F::cast_from(0.19756347548806534796e1_f64) * t112706 * t1829 + F::cast_from(0.52041769129231196772e1_f64) * t105365 * t30850 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t2142 * t24697 - F::cast_from(0.13010442282307799193e1_f64) * t29129 * t30874 - F::cast_from(0.26020884564615598386e1_f64) * t104529 * t30878 - F::cast_from(0.39512695097613069591e1_f64) * t112902 * t1775 - F::cast_from(0.52041769129231196772e1_f64) * t29275 * t30744 + F::cast_from(0.10408353825846239354e2_f64) * t29275 * t30907 - F::cast_from(0.78062653693846795158e1_f64) * t97348 * t30767 * t1794 * t1287;
    t116607
}
