//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2254/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2254(t29135: f64, t3566: f64, t8190: f64, t29109: f64, t460: f64, t5251: f64, t8945: f64, t3601: f64, t8197: f64, t1248: f64, t1287: f64, t1294: f64, t1295: f64, t17944: f64, t1829: f64, t26889: f64, t26891: f64, t26918: f64, t26969: f64, t26996: f64, t29132: f64, t29158: f64, t29174: f64, t29178: f64, t29187: f64, t29204: f64, t29247: f64, t29251: f64, t3569: f64, t3588: f64, t3769: f64, t3783: f64, t3790: f64, t7643: f64, t7651: f64, t7652: f64, t7666: f64, t8201: f64, t96938: f64, t96979: f64, t97041: f64, t97318: f64) -> f64 {
    let t105509 = t3566 * t29135;
    let t105512 = t3566 * t8190;
    let t105519 = t460 * t29109;
    let t105530 = t5251 * t8945;
    let t105540 = t8197 * t3601;
    let t105553 = -0.8673628188205199462e0_f64 * t26918 * t29247 - 0.8673628188205199462e0_f64 * t26918 * t29251 + 0.34694512752820797848e1_f64 * t105509 * t26996 + 0.13170898365871023197e1_f64 * t105512 * t3569 - 0.17347256376410398924e1_f64 * t29204 * t29187 - 0.8673628188205199462e0_f64 * t29132 * t7666 - 0.13170898365871023197e1_f64 * t105519 * t1295 - 0.52041769129231196772e1_f64 * t7651 * t26969 * t29178 * t1294 - 0.17347256376410398924e1_f64 * t7643 * t7652 * t8201 * t3790 - 0.17347256376410398924e1_f64 * t105530 * t26891 - 0.26020884564615598386e1_f64 * t97041 * t29158 * t17944 - 0.17347256376410398924e1_f64 * t26889 * t29174 * t1248 * t1287 - 0.17347256376410398924e1_f64 * t96979 * t105540 * t3769 + 0.8673628188205199462e0_f64 * t97318 * t105540 * t3783 - 0.8673628188205199462e0_f64 * t26889 * t8197 * t3588 * t1287 - 0.65854491829355115987e0_f64 * t96938 * t1829;
    t105553
}
