//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2236/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2236<F: Float>(t3303: F, t5341: F, t5333: F, t104505: F, t105365: F, t111845: F, t1203: F, t1287: F, t1769: F, t1794: F, t20728: F, t26895: F, t26922: F, t26969: F, t26979: F, t26994: F, t29109: F, t29136: F, t29141: F, t29163: F, t29166: F, t29237: F, t29278: F, t29283: F, t30747: F, t30748: F, t30763: F, t30849: F, t30853: F, t5284: F, t5497: F, t6574: F, t7602: F, t7636: F, t7637: F, t7651: F, t8208: F, t96953: F, t96979: F, t97019: F, t97304: F, t97318: F) -> (F, F, F) {
    let t111987 = t3303 * t5341;
    let t111991 = t3303 * t5333;
    let t112009 = F::cast_from(0.17347256376410398924e1_f64) * t26895 * t29278 * t1794 * t1287 - F::cast_from(0.52041769129231196772e1_f64) * t7651 * t26969 * t8208 * t5497 + F::cast_from(0.13170898365871023197e1_f64) * t97019 * t6574 - F::cast_from(0.34694512752820797848e1_f64) * t29136 * t29237 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t30763 * t104505 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t8208 * t5284 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7637 * t29109 * t1769 + F::cast_from(0.17347256376410398924e1_f64) * t26979 * t30748 + F::cast_from(0.65854491829355115987e0_f64) * t7602 * t20728 - F::cast_from(0.34694512752820797848e1_f64) * t96979 * t30853 * t111987 + F::cast_from(0.17347256376410398924e1_f64) * t97318 * t30853 * t111991 + F::cast_from(0.34694512752820797848e1_f64) * t97304 * t30849 * t104505 + F::cast_from(0.17347256376410398924e1_f64) * t29141 * t29283 + F::cast_from(0.8673628188205199462e0_f64) * t26922 * t111845 * t29166 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t30747 * t1203 + F::cast_from(0.17347256376410398924e1_f64) * t105365 * t29163;
    (t111987, t111991, t112009)
}
