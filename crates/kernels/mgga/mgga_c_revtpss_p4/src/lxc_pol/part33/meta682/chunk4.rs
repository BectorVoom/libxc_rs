//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2237/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2237<F: Float>(t20849: F, t2142: F, t1276: F, t2148: F, t3140: F, t6695: F, t105509: F, t105512: F, t105530: F, t105558: F, t1203: F, t1214: F, t1215: F, t27008: F, t29136: F, t29217: F, t29220: F, t29268: F, t29275: F, t29279: F, t29287: F, t29293: F, t29301: F, t29308: F, t30739: F, t30757: F, t5231: F, t5237: F, t6703: F, t7637: F, t7662: F, t96927: F, t96954: F, t97358: F, t97475: F) -> F {
    let t112018 = t20849 * t2142;
    let t112048 = t2148 * t6695 * t3140 * t1276;
    let t112051 = F::cast_from(0.17347256376410398924e1_f64) * t105558 * t29308 + F::cast_from(0.34694512752820797848e1_f64) * t105509 * t29268 + F::cast_from(0.26341796731742046394e1_f64) * t105512 * t5231 - F::cast_from(0.65854491829355115987e0_f64) * t112018 * t1215 - F::cast_from(0.34694512752820797848e1_f64) * t29136 * t29293 - F::cast_from(0.17347256376410398924e1_f64) * t105530 * t29217 + F::cast_from(0.13170898365871023197e1_f64) * t29220 * t5237 - F::cast_from(0.17347256376410398924e1_f64) * t29275 * t29301 - F::cast_from(0.34694512752820797848e1_f64) * t96927 * t30757 * t96954 - F::cast_from(0.52041769129231196772e1_f64) * t97475 * t7637 * t30739 * t1203 + F::cast_from(0.10408353825846239354e2_f64) * t97358 * t7637 * t30739 * t1214 + F::cast_from(0.34694512752820797848e1_f64) * t105509 * t29287 + F::cast_from(0.17347256376410398924e1_f64) * t29136 * t29279 + F::cast_from(0.13170898365871023197e1_f64) * t27008 * t6703 - F::cast_from(0.4336814094102599731e0_f64) * t112048 * t7662;
    t112051
}
