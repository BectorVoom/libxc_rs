//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2237/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2237(t20849: f64, t2142: f64, t1276: f64, t2148: f64, t3140: f64, t6695: f64, t105509: f64, t105512: f64, t105530: f64, t105558: f64, t1203: f64, t1214: f64, t1215: f64, t27008: f64, t29136: f64, t29217: f64, t29220: f64, t29268: f64, t29275: f64, t29279: f64, t29287: f64, t29293: f64, t29301: f64, t29308: f64, t30739: f64, t30757: f64, t5231: f64, t5237: f64, t6703: f64, t7637: f64, t7662: f64, t96927: f64, t96954: f64, t97358: f64, t97475: f64) -> f64 {
    let t112018 = t20849 * t2142;
    let t112048 = t2148 * t6695 * t3140 * t1276;
    let t112051 = 0.17347256376410398924e1_f64 * t105558 * t29308 + 0.34694512752820797848e1_f64 * t105509 * t29268 + 0.26341796731742046394e1_f64 * t105512 * t5231 - 0.65854491829355115987e0_f64 * t112018 * t1215 - 0.34694512752820797848e1_f64 * t29136 * t29293 - 0.17347256376410398924e1_f64 * t105530 * t29217 + 0.13170898365871023197e1_f64 * t29220 * t5237 - 0.17347256376410398924e1_f64 * t29275 * t29301 - 0.34694512752820797848e1_f64 * t96927 * t30757 * t96954 - 0.52041769129231196772e1_f64 * t97475 * t7637 * t30739 * t1203 + 0.10408353825846239354e2_f64 * t97358 * t7637 * t30739 * t1214 + 0.34694512752820797848e1_f64 * t105509 * t29287 + 0.17347256376410398924e1_f64 * t29136 * t29279 + 0.13170898365871023197e1_f64 * t27008 * t6703 - 0.4336814094102599731e0_f64 * t112048 * t7662;
    t112051
}
