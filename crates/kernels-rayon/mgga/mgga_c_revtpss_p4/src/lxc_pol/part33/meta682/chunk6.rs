//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2239/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2239(t1794: f64, t8190: f64, t73: f64, t30881: f64, t3565: f64, t7635: f64, t104524: f64, t105350: f64, t105579: f64, t1214: f64, t1294: f64, t1829: f64, t20741: f64, t21332: f64, t2142: f64, t26895: f64, t26918: f64, t26922: f64, t26949: f64, t26969: f64, t26979: f64, t29129: f64, t29141: f64, t29166: f64, t29251: f64, t29272: f64, t30736: f64, t30747: f64, t30840: f64, t30893: f64, t5231: f64, t5458: f64, t6702: f64, t7602: f64, t7627: f64, t7636: f64, t7637: f64, t7645: f64, t7651: f64, t7652: f64, t8202: f64) -> (f64, f64, f64) {
    let t112120 = t8190 * t1794;
    let t112121 = t112120 * t73;
    let t112129 = t30881 * t3565 * t7635;
    let t112138 = 0.17347256376410398924e1_f64 * t29141 * t29272 - 0.13170898365871023197e1_f64 * t7602 * t20741 - 0.8673628188205199462e0_f64 * t29129 * t29251 - 0.8673628188205199462e0_f64 * t26918 * t30893 + 0.8673628188205199462e0_f64 * t26979 * t30736 - 0.8673628188205199462e0_f64 * t7636 * t7637 * t2142 * t21332 - 0.26020884564615598386e1_f64 * t7651 * t26969 * t7627 * t6702 - 0.13170898365871023197e1_f64 * t104524 * t1829 + 0.26341796731742046394e1_f64 * t105579 * t5231 + 0.8673628188205199462e0_f64 * t7651 * t7652 * t30840 * t1294 + 0.17347256376410398924e1_f64 * t26895 * t112121 * t5458 + 0.17347256376410398924e1_f64 * t26922 * t112121 * t29166 + 0.17347256376410398924e1_f64 * t112129 * t7645 - 0.52041769129231196772e1_f64 * t26949 * t7637 * t30747 * t1214 + 0.17347256376410398924e1_f64 * t105350 * t8202;
    (t112120, t112121, t112138)
}
