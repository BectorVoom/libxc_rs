//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2225/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225(t104490: f64, t1203: f64, t1204: f64, t17331: f64, t2144: f64, t26918: f64, t26922: f64, t26937: f64, t26969: f64, t26994: f64, t26999: f64, t27011: f64, t29118: f64, t29124: f64, t29149: f64, t29166: f64, t29183: f64, t29227: f64, t3551: f64, t3568: f64, t3738: f64, t3791: f64, t5216: f64, t5231: f64, t5423: f64, t7629: f64, t7637: f64, t7651: f64, t8190: f64, t8197: f64, t8198: f64, t8201: f64, t97019: f64, t97078: f64, t97475: f64) -> f64 {
    let t104601 = -0.52041769129231196772e1_f64 * t97475 * t7637 * t8197 * t3568 - 0.52041769129231196772e1_f64 * t26937 * t29149 - 0.8673628188205199462e0_f64 * t97078 * t8198 + 0.26341796731742046394e1_f64 * t97019 * t5231 + 0.34694512752820797848e1_f64 * t26994 * t7637 * t29118 * t1203 + 0.17347256376410398924e1_f64 * t26994 * t7637 * t8201 * t3551 + 0.13170898365871023197e1_f64 * t1204 * t29183 + 0.17347256376410398924e1_f64 * t26922 * t104490 * t29166 + 0.13170898365871023197e1_f64 * t27011 * t5423 + 0.13170898365871023197e1_f64 * t5216 * t7629 + 0.65854491829355115987e0_f64 * t17331 * t2144 - 0.8673628188205199462e0_f64 * t26918 * t29124 - 0.26020884564615598386e1_f64 * t7651 * t26969 * t8190 * t3738 + 0.13170898365871023197e1_f64 * t26999 * t5423 - 0.65854491829355115987e0_f64 * t29227 * t3791;
    t104601
}
