//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2250/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2250<F: Float>(t105202: F, t7642: F, t104480: F, t1243: F, t2149: F, t104483: F, t1214: F, t1248: F, t1287: F, t1294: F, t17330: F, t1775: F, t17951: F, t17999: F, t2142: F, t26922: F, t26945: F, t26969: F, t26994: F, t26999: F, t29141: F, t29204: F, t29207: F, t29220: F, t29224: F, t29268: F, t29271: F, t29282: F, t29292: F, t29300: F, t3551: F, t3585: F, t3791: F, t5246: F, t7602: F, t7636: F, t7637: F, t7645: F, t7651: F, t7652: F, t8208: F, t96927: F, t96954: F, t97343: F, t97370: F) -> F {
    let t105350 = t7642 * t105202;
    let t105354 = t2149 * t104480 * t1243;
    let t105358 = -F::cast_from(0.34694512752820797848e1_f64) * t96927 * t29292 * t96954 + F::cast_from(0.65854491829355115987e0_f64) * t7602 * t17999 + F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7652 * t8208 * t3551 - F::cast_from(0.65854491829355115987e0_f64) * t29220 * t3585 - F::cast_from(0.65854491829355115987e0_f64) * t97370 * t1775 + F::cast_from(0.34694512752820797848e1_f64) * t29204 * t29224 + F::cast_from(0.34694512752820797848e1_f64) * t97343 * t29268 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29300 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t29282 * t1248 * t1287 - F::cast_from(0.52041769129231196772e1_f64) * t7651 * t26969 * t29271 * t1294 - F::cast_from(0.65854491829355115987e0_f64) * t29207 * t3791 - F::cast_from(0.13170898365871023197e1_f64) * t26999 * t5246 + F::cast_from(0.17347256376410398924e1_f64) * t29141 * t26945 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t2142 * t17330 + F::cast_from(0.17347256376410398924e1_f64) * t105350 * t7645 - F::cast_from(0.4336814094102599731e0_f64) * t105354 * t104483 * t17951;
    t105358
}
