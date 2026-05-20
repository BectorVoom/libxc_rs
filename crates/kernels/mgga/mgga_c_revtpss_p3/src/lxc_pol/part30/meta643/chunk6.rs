//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2255/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2255<F: Float>(t26921: F, t8205: F, t2143: F, t3566: F, t17306: F, t2142: F, t3556: F, t8945: F, t104490: F, t104504: F, t104510: F, t1287: F, t17170: F, t1794: F, t17975: F, t26889: F, t26924: F, t26931: F, t26969: F, t26994: F, t29158: F, t29159: F, t29195: F, t29216: F, t29217: F, t3551: F, t3569: F, t3584: F, t3790: F, t5284: F, t5352: F, t5457: F, t5479: F, t7636: F, t7637: F, t7651: F, t7659: F, t7660: F, t8190: F, t8197: F, t8208: F, t96928: F, t96953: F, t97067: F, t97095: F, t97304: F, t97308: F, t97318: F, t97397: F) -> F {
    let t105558 = t8205 * t26921;
    let t105576 = t3566 * t2143;
    let t105579 = t17306 * t2142;
    let t105598 = t3556 * t8945;
    let t105613 = F::cast_from(0.34694512752820797848e1_f64) * t97304 * t29216 * t104510 + F::cast_from(0.17347256376410398924e1_f64) * t105558 * t26924 + F::cast_from(0.17347256376410398924e1_f64) * t26994 * t7637 * t8197 * t3584 - F::cast_from(0.17347256376410398924e1_f64) * t97397 * t29195 * t5479 * t96928 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t26969 * t8208 * t3790 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t104490 * t29159 - F::cast_from(0.26341796731742046394e1_f64) * t105576 * t17975 + F::cast_from(0.13170898365871023197e1_f64) * t105579 * t3569 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t29158 * t5457 * t97067 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t97095 * t1794 * t1287 - F::cast_from(0.8673628188205199462e0_f64) * t7659 * t26931 * t5284 * t1287 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t7660 * t17170 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t105598 * t29217 + F::cast_from(0.17347256376410398924e1_f64) * t97318 * t29195 * t5479 * t104504 - F::cast_from(0.17347256376410398924e1_f64) * t97308 * t29195 * t5479 * t5352 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t8190 * t3551;
    t105613
}
