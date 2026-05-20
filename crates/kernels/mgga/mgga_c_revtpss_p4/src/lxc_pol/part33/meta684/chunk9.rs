//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2260/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2260<F: Float>(t112120: F, t3153: F, t1243: F, t30840: F, t1248: F, t1287: F, t1294: F, t1828: F, t20710: F, t20900: F, t26895: F, t26906: F, t26931: F, t26937: F, t29124: F, t29129: F, t29194: F, t29200: F, t29278: F, t30735: F, t30751: F, t30772: F, t30860: F, t30878: F, t3769: F, t3783: F, t5465: F, t5480: F, t5497: F, t6628: F, t7602: F, t7636: F, t7643: F, t7651: F, t7652: F, t7659: F, t7660: F, t8190: F, t96883: F, t97332: F) -> F {
    let t112651 = t112120 * t3153;
    let t112686 = t1243 * t30840;
    let t112697 = F::cast_from(0.8673628188205199462e0_f64) * t26895 * t30735 * t1248 * t1287 + F::cast_from(0.8673628188205199462e0_f64) * t29200 * t112651 * t5480 - F::cast_from(0.17347256376410398924e1_f64) * t29194 * t112651 * t5465 + F::cast_from(0.17347256376410398924e1_f64) * t7651 * t7652 * t8190 * t5497 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t7660 * t20900 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t96883 * t30878 - F::cast_from(0.8673628188205199462e0_f64) * t26906 * t97332 * t6628 * t3769 + F::cast_from(0.4336814094102599731e0_f64) * t96883 * t30860 + F::cast_from(0.4336814094102599731e0_f64) * t26906 * t26931 * t6628 * t3783 + F::cast_from(0.65854491829355115987e0_f64) * t7602 * t20710 + F::cast_from(0.8673628188205199462e0_f64) * t26937 * t30772 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29278 * t1828 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t112686 * t1248 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7652 * t30751 * t1294 - F::cast_from(0.8673628188205199462e0_f64) * t29129 * t29124;
    t112697
}
