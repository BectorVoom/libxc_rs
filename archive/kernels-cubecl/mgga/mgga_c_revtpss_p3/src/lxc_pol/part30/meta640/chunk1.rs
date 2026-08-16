//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2225/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2225<F: Float>(t104490: F, t1203: F, t1204: F, t17331: F, t2144: F, t26918: F, t26922: F, t26937: F, t26969: F, t26994: F, t26999: F, t27011: F, t29118: F, t29124: F, t29149: F, t29166: F, t29183: F, t29227: F, t3551: F, t3568: F, t3738: F, t3791: F, t5216: F, t5231: F, t5423: F, t7629: F, t7637: F, t7651: F, t8190: F, t8197: F, t8198: F, t8201: F, t97019: F, t97078: F, t97475: F) -> F {
    let t104601 = -F::cast_from(0.52041769129231196772e1_f64) * t97475 * t7637 * t8197 * t3568 - F::cast_from(0.52041769129231196772e1_f64) * t26937 * t29149 - F::cast_from(0.8673628188205199462e0_f64) * t97078 * t8198 + F::cast_from(0.26341796731742046394e1_f64) * t97019 * t5231 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29118 * t1203 + F::cast_from(0.17347256376410398924e1_f64) * t26994 * t7637 * t8201 * t3551 + F::cast_from(0.13170898365871023197e1_f64) * t1204 * t29183 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t104490 * t29166 + F::cast_from(0.13170898365871023197e1_f64) * t27011 * t5423 + F::cast_from(0.13170898365871023197e1_f64) * t5216 * t7629 + F::cast_from(0.65854491829355115987e0_f64) * t17331 * t2144 - F::cast_from(0.8673628188205199462e0_f64) * t26918 * t29124 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t26969 * t8190 * t3738 + F::cast_from(0.13170898365871023197e1_f64) * t26999 * t5423 - F::cast_from(0.65854491829355115987e0_f64) * t29227 * t3791;
    t104601
}
