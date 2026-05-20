//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2265/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2265<F: Float>(t5219: F, t8190: F, t30882: F, t7635: F, t105433: F, t105598: F, t112535: F, t1214: F, t1215: F, t1248: F, t1287: F, t1294: F, t1774: F, t1775: F, t1794: F, t20748: F, t2149: F, t2150: F, t26895: F, t26922: F, t26969: F, t26994: F, t29118: F, t29132: F, t29141: F, t29179: F, t29186: F, t29220: F, t30767: F, t30771: F, t30840: F, t30854: F, t30886: F, t473: F, t5246: F, t5284: F, t7637: F, t7643: F, t7651: F, t7654: F, t8201: F, t8217: F, t96861: F, t97348: F) -> F {
    let t112902 = t5219 * t8190;
    let t112943 = t30882 * t7635;
    let t112950 = -F::cast_from(0.39512695097613069591e1_f64) * t96861 * t20748 - F::cast_from(0.13170898365871023197e1_f64) * t112902 * t1215 + F::cast_from(0.8673628188205199462e0_f64) * t26922 * t30771 * t1248 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t105598 * t30854 - F::cast_from(0.52041769129231196772e1_f64) * t7651 * t26969 * t30886 * t1294 - F::cast_from(0.13170898365871023197e1_f64) * t29220 * t5246 - F::cast_from(0.26020884564615598386e1_f64) * t97348 * t30767 * t1248 * t1287 - F::cast_from(0.13170898365871023197e1_f64) * t105433 * t1775 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29186 * t1774 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t29118 * t1794 * t1287 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t8201 * t5284 * t1287 - F::cast_from(0.4336814094102599731e0_f64) * t2149 * t2150 * t473 * t112535 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t30840 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t112943 * t7654 + F::cast_from(0.17347256376410398924e1_f64) * t29141 * t29179 - F::cast_from(0.8673628188205199462e0_f64) * t29132 * t8217;
    t112950
}
