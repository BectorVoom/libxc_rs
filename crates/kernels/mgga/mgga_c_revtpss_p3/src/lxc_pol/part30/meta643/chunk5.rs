//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2254/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2254<F: Float>(t29135: F, t3566: F, t8190: F, t29109: F, t460: F, t5251: F, t8945: F, t3601: F, t8197: F, t1248: F, t1287: F, t1294: F, t1295: F, t17944: F, t1829: F, t26889: F, t26891: F, t26918: F, t26969: F, t26996: F, t29132: F, t29158: F, t29174: F, t29178: F, t29187: F, t29204: F, t29247: F, t29251: F, t3569: F, t3588: F, t3769: F, t3783: F, t3790: F, t7643: F, t7651: F, t7652: F, t7666: F, t8201: F, t96938: F, t96979: F, t97041: F, t97318: F) -> F {
    let t105509 = t3566 * t29135;
    let t105512 = t3566 * t8190;
    let t105519 = t460 * t29109;
    let t105530 = t5251 * t8945;
    let t105540 = t8197 * t3601;
    let t105553 = -F::cast_from(0.8673628188205199462e0_f64) * t26918 * t29247 - F::cast_from(0.8673628188205199462e0_f64) * t26918 * t29251 + F::cast_from(0.34694512752820797848e1_f64) * t105509 * t26996 + F::cast_from(0.13170898365871023197e1_f64) * t105512 * t3569 - F::cast_from(0.17347256376410398924e1_f64) * t29204 * t29187 - F::cast_from(0.8673628188205199462e0_f64) * t29132 * t7666 - F::cast_from(0.13170898365871023197e1_f64) * t105519 * t1295 - F::cast_from(0.52041769129231196772e1_f64) * t7651 * t26969 * t29178 * t1294 - F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7652 * t8201 * t3790 - F::cast_from(0.17347256376410398924e1_f64) * t105530 * t26891 - F::cast_from(0.26020884564615598386e1_f64) * t97041 * t29158 * t17944 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t29174 * t1248 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t96979 * t105540 * t3769 + F::cast_from(0.8673628188205199462e0_f64) * t97318 * t105540 * t3783 - F::cast_from(0.8673628188205199462e0_f64) * t26889 * t8197 * t3588 * t1287 - F::cast_from(0.65854491829355115987e0_f64) * t96938 * t1829;
    t105553
}
