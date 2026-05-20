//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2263/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2263<F: Float>(t1769: F, t1774: F, t30882: F, t7658: F, t105167: F, t105269: F, t1214: F, t1248: F, t1287: F, t1294: F, t1794: F, t2151: F, t26889: F, t26895: F, t26922: F, t26931: F, t26969: F, t29122: F, t29186: F, t29207: F, t29271: F, t30747: F, t30771: F, t30854: F, t30867: F, t30886: F, t5284: F, t5429: F, t6587: F, t6622: F, t7627: F, t7637: F, t7643: F, t7651: F, t7652: F, t7659: F, t7662: F, t97066: F, t97343: F, t97363: F) -> F {
    let t112822 = t1769 * t1774;
    let t112843 = t30882 * t7658;
    let t112846 = -F::cast_from(0.4336814094102599731e0_f64) * t7659 * t26931 * t6622 * t1287 + F::cast_from(0.34694512752820797848e1_f64) * t97343 * t30867 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t29271 * t1794 * t1287 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t26969 * t30771 * t1294 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t7627 * t6587 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t30886 * t1248 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t30747 * t1248 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t29186 * t1794 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t97363 * t30854 + F::cast_from(0.26341796731742046394e1_f64) * t29207 * t5429 - F::cast_from(0.10408353825846239354e2_f64) * t105269 * t2151 * t112822 * t1214 - F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t112822 * t1294 - F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7652 * t30771 * t1214 - F::cast_from(0.8673628188205199462e0_f64) * t7659 * t105167 * t1794 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t7659 * t29122 * t5284 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t112843 * t7662;
    t112846
}
