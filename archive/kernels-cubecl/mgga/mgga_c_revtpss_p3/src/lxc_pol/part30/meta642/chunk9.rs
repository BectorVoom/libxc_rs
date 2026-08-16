//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2246/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2246<F: Float>(t3601: F, t8208: F, t1794: F, t7627: F, t3153: F, t3555: F, t8190: F, t105021: F, t1214: F, t1215: F, t1294: F, t18030: F, t18070: F, t18073: F, t225: F, t26933: F, t26971: F, t26976: F, t29109: F, t29129: F, t29141: F, t29193: F, t29194: F, t29196: F, t29207: F, t29216: F, t29278: F, t3739: F, t3769: F, t3783: F, t460: F, t494: F, t5465: F, t7637: F, t7643: F, t7648: F, t7652: F, t8202: F, t96861: F, t96870: F, t96929: F, t96953: F, t97313: F, t97397: F) -> (F, F, F) {
    let t105114 = t8208 * t3601;
    let t105121 = t7627 * t1794;
    let t105122 = t105121 * t3153;
    let t105134 = t3555 * t8190;
    let t105155 = F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7637 * t29109 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t96870 * t8202 - F::cast_from(0.8673628188205199462e0_f64) * t97397 * t105114 * t3783 - F::cast_from(0.17347256376410398924e1_f64) * t7648 * t29193 * t29196 - F::cast_from(0.17347256376410398924e1_f64) * t29194 * t105122 * t5465 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t105021 * t225 * t494 + F::cast_from(0.26341796731742046394e1_f64) * t26976 * t18070 + F::cast_from(0.13170898365871023197e1_f64) * t26976 * t18073 - F::cast_from(0.13170898365871023197e1_f64) * t105134 * t1215 + F::cast_from(0.17347256376410398924e1_f64) * t97313 * t105114 * t3769 + F::cast_from(0.13170898365871023197e1_f64) * t29207 * t3739 - F::cast_from(0.8673628188205199462e0_f64) * t29129 * t26933 - F::cast_from(0.39512695097613069591e1_f64) * t96861 * t18030 + F::cast_from(0.34694512752820797848e1_f64) * t96953 * t29216 * t96929 - F::cast_from(0.26020884564615598386e1_f64) * t29141 * t26971 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29278 * t1294;
    (t105121, t105122, t105155)
}
