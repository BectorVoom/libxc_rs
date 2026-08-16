//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2260/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2260(t112120: f64, t3153: f64, t1243: f64, t30840: f64, t1248: f64, t1287: f64, t1294: f64, t1828: f64, t20710: f64, t20900: f64, t26895: f64, t26906: f64, t26931: f64, t26937: f64, t29124: f64, t29129: f64, t29194: f64, t29200: f64, t29278: f64, t30735: f64, t30751: f64, t30772: f64, t30860: f64, t30878: f64, t3769: f64, t3783: f64, t5465: f64, t5480: f64, t5497: f64, t6628: f64, t7602: f64, t7636: f64, t7643: f64, t7651: f64, t7652: f64, t7659: f64, t7660: f64, t8190: f64, t96883: f64, t97332: f64) -> f64 {
    let t112651 = t112120 * t3153;
    let t112686 = t1243 * t30840;
    let t112697 = 0.8673628188205199462e0_f64 * t26895 * t30735 * t1248 * t1287 + 0.8673628188205199462e0_f64 * t29200 * t112651 * t5480 - 0.17347256376410398924e1_f64 * t29194 * t112651 * t5465 + 0.17347256376410398924e1_f64 * t7651 * t7652 * t8190 * t5497 - 0.4336814094102599731e0_f64 * t7659 * t7660 * t20900 * t1287 - 0.8673628188205199462e0_f64 * t96883 * t30878 - 0.8673628188205199462e0_f64 * t26906 * t97332 * t6628 * t3769 + 0.4336814094102599731e0_f64 * t96883 * t30860 + 0.4336814094102599731e0_f64 * t26906 * t26931 * t6628 * t3783 + 0.65854491829355115987e0_f64 * t7602 * t20710 + 0.8673628188205199462e0_f64 * t26937 * t30772 - 0.34694512752820797848e1_f64 * t7643 * t7652 * t29278 * t1828 - 0.4336814094102599731e0_f64 * t7659 * t112686 * t1248 * t1287 + 0.17347256376410398924e1_f64 * t7636 * t7652 * t30751 * t1294 - 0.8673628188205199462e0_f64 * t29129 * t29124;
    t112697
}
