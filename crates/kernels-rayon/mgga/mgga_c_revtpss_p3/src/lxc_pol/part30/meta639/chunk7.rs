//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2223/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2223(t1203: f64, t471: f64, t355: f64, t104465: f64, t104473: f64, t104482: f64, t104483: f64, t104490: f64, t1214: f64, t12713: f64, t1294: f64, t1295: f64, t16750: f64, t17848: f64, t17875: f64, t17963: f64, t2142: f64, t26889: f64, t26895: f64, t26988: f64, t26994: f64, t29141: f64, t29158: f64, t29174: f64, t29194: f64, t29195: f64, t29200: f64, t29212: f64, t3551: f64, t3738: f64, t5457: f64, t5458: f64, t5465: f64, t7636: f64, t7637: f64, t7643: f64, t7651: f64, t7652: f64, t8202: f64, t97034: f64, t97304: f64, t97348: f64) -> (f64, f64) {
    let t104504 = t471 * t1203;
    let t104505 = t355 * t104504;
    let t104509 = 0.34694512752820797848e1_f64 * t7636 * t7652 * t29174 * t1294 + 0.8673628188205199462e0_f64 * t29141 * t26988 + 0.8673628188205199462e0_f64 * t7643 * t7637 * t2142 * t16750 + 0.34694512752820797848e1_f64 * t26994 * t7637 * t29174 * t1214 - 0.13170898365871023197e1_f64 * t104465 * t1295 + 0.8673628188205199462e0_f64 * t7651 * t7652 * t2142 * t17963 - 0.17347256376410398924e1_f64 * t29194 * t104473 * t5465 - 0.8673628188205199462e0_f64 * t29194 * t29195 * t12713 - 0.26020884564615598386e1_f64 * t104482 * t104483 * t17848 + 0.4336814094102599731e0_f64 * t29200 * t29195 * t17875 + 0.17347256376410398924e1_f64 * t26895 * t104490 * t5458 - 0.26020884564615598386e1_f64 * t97348 * t29158 * t5457 * t3738 - 0.8673628188205199462e0_f64 * t26889 * t29158 * t5457 * t3551 + 0.8673628188205199462e0_f64 * t97034 * t8202 + 0.34694512752820797848e1_f64 * t97304 * t29212 * t104505;
    (t104504, t104509)
}
