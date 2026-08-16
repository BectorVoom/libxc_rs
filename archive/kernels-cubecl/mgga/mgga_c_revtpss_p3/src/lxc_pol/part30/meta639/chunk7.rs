//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2223/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223<F: Float>(t1203: F, t471: F, t355: F, t104465: F, t104473: F, t104482: F, t104483: F, t104490: F, t1214: F, t12713: F, t1294: F, t1295: F, t16750: F, t17848: F, t17875: F, t17963: F, t2142: F, t26889: F, t26895: F, t26988: F, t26994: F, t29141: F, t29158: F, t29174: F, t29194: F, t29195: F, t29200: F, t29212: F, t3551: F, t3738: F, t5457: F, t5458: F, t5465: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t8202: F, t97034: F, t97304: F, t97348: F) -> (F, F) {
    let t104504 = t471 * t1203;
    let t104505 = t355 * t104504;
    let t104509 = F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29174 * t1294 + F::cast_from(0.8673628188205199462e0_f64) * t29141 * t26988 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t2142 * t16750 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29174 * t1214 - F::cast_from(0.13170898365871023197e1_f64) * t104465 * t1295 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t2142 * t17963 - F::cast_from(0.17347256376410398924e1_f64) * t29194 * t104473 * t5465 - F::cast_from(0.8673628188205199462e0_f64) * t29194 * t29195 * t12713 - F::cast_from(0.26020884564615598386e1_f64) * t104482 * t104483 * t17848 + F::cast_from(0.4336814094102599731e0_f64) * t29200 * t29195 * t17875 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t104490 * t5458 - F::cast_from(0.26020884564615598386e1_f64) * t97348 * t29158 * t5457 * t3738 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t29158 * t5457 * t3551 + F::cast_from(0.8673628188205199462e0_f64) * t97034 * t8202 + F::cast_from(0.34694512752820797848e1_f64) * t97304 * t29212 * t104505;
    (t104504, t104509)
}
