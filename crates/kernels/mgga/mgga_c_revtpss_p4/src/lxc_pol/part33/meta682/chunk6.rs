//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2239/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2239<F: Float>(t1794: F, t8190: F, t73: F, t30881: F, t3565: F, t7635: F, t104524: F, t105350: F, t105579: F, t1214: F, t1294: F, t1829: F, t20741: F, t21332: F, t2142: F, t26895: F, t26918: F, t26922: F, t26949: F, t26969: F, t26979: F, t29129: F, t29141: F, t29166: F, t29251: F, t29272: F, t30736: F, t30747: F, t30840: F, t30893: F, t5231: F, t5458: F, t6702: F, t7602: F, t7627: F, t7636: F, t7637: F, t7645: F, t7651: F, t7652: F, t8202: F) -> (F, F, F) {
    let t112120 = t8190 * t1794;
    let t112121 = t112120 * t73;
    let t112129 = t30881 * t3565 * t7635;
    let t112138 = F::cast_from(0.17347256376410398924e1_f64) * t29141 * t29272 - F::cast_from(0.13170898365871023197e1_f64) * t7602 * t20741 - F::cast_from(0.8673628188205199462e0_f64) * t29129 * t29251 - F::cast_from(0.8673628188205199462e0_f64) * t26918 * t30893 + F::cast_from(0.8673628188205199462e0_f64) * t26979 * t30736 - F::cast_from(0.8673628188205199462e0_f64) * t7636 * t7637 * t2142 * t21332 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t26969 * t7627 * t6702 - F::cast_from(0.13170898365871023197e1_f64) * t104524 * t1829 + F::cast_from(0.26341796731742046394e1_f64) * t105579 * t5231 + F::cast_from(0.8673628188205199462e0_f64) * t7651 * t7652 * t30840 * t1294 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t112121 * t5458 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t112121 * t29166 + F::cast_from(0.17347256376410398924e1_f64) * t112129 * t7645 - F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7637 * t30747 * t1214 + F::cast_from(0.17347256376410398924e1_f64) * t105350 * t8202;
    (t112120, t112121, t112138)
}
