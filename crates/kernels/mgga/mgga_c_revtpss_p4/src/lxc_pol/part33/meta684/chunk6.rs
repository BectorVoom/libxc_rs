//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2257/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2257<F: Float>(t104510: F, t105284: F, t112121: F, t112535: F, t1203: F, t1214: F, t1828: F, t1829: F, t21342: F, t21348: F, t2148: F, t2152: F, t225: F, t26889: F, t26949: F, t26969: F, t26994: F, t29109: F, t29119: F, t29136: F, t29141: F, t29149: F, t29159: F, t29199: F, t29201: F, t30751: F, t30767: F, t30771: F, t30849: F, t460: F, t494: F, t5245: F, t5497: F, t6564: F, t7629: F, t7632: F, t7636: F, t7637: F, t7643: F, t7651: F, t7652: F, t8201: F, t8205: F, t97041: F) -> F {
    let t112564 = F::cast_from(0.17347256376410398924e1_f64) * t7651 * t7652 * t29109 * t1828 - F::cast_from(0.52041769129231196772e1_f64) * t97041 * t30849 * t104510 - F::cast_from(0.4336814094102599731e0_f64) * t2148 * t21342 * t2152 + F::cast_from(0.65854491829355115987e0_f64) * t6564 * t7629 + F::cast_from(0.8673628188205199462e0_f64) * t8205 * t29199 * t29201 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t112121 * t29159 - F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7637 * t8201 * t5245 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t112535 * t225 * t494 + F::cast_from(0.17347256376410398924e1_f64) * t29136 * t29119 - F::cast_from(0.13170898365871023197e1_f64) * t105284 * t1829 - F::cast_from(0.39512695097613069591e1_f64) * t7632 * t21348 + F::cast_from(0.52041769129231196772e1_f64) * t7643 * t26969 * t30767 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t26994 * t7637 * t30751 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7652 * t30771 * t1203 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t8201 * t5497 - F::cast_from(0.52041769129231196772e1_f64) * t29141 * t29149;
    t112564
}
