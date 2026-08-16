//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2247/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2247<F: Float>(t1243: F, t29109: F, t105121: F, t73: F, t1032: F, t5412: F, t2148: F, t1214: F, t1248: F, t12657: F, t1287: F, t17968: F, t1829: F, t26889: F, t26895: F, t26922: F, t26949: F, t26963: F, t26999: F, t29118: F, t29159: F, t29167: F, t29271: F, t29275: F, t3584: F, t3790: F, t5237: F, t5458: F, t7632: F, t7635: F, t7636: F, t7637: F, t7643: F, t7652: F, t7654: F, t7659: F, t8197: F, t8198: F, t8201: F, t8208: F, t97422: F, t97425: F) -> (F, F, F) {
    let t105167 = t1243 * t29109;
    let t105193 = t105121 * t73;
    let t105202 = t5412 * t1032;
    let t105203 = t2148 * t105202;
    let t105206 = F::cast_from(0.17347256376410398924e1_f64) * t26922 * t29271 * t1248 * t1287 - F::cast_from(0.39512695097613069591e1_f64) * t7632 * t17968 + F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7652 * t8197 * t3790 - F::cast_from(0.8673628188205199462e0_f64) * t7659 * t105167 * t1248 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t7643 * t7652 * t8208 * t3584 - F::cast_from(0.8673628188205199462e0_f64) * t12657 * t7635 * t8198 + F::cast_from(0.13170898365871023197e1_f64) * t26999 * t5237 - F::cast_from(0.52041769129231196772e1_f64) * t26949 * t7637 * t29118 * t1214 - F::cast_from(0.26020884564615598386e1_f64) * t26949 * t7637 * t8201 * t3584 - F::cast_from(0.65854491829355115987e0_f64) * t97425 * t1829 + F::cast_from(0.17347256376410398924e1_f64) * t97422 * t29167 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t105193 * t29159 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t105193 * t5458 - F::cast_from(0.8673628188205199462e0_f64) * t29275 * t26963 + F::cast_from(0.17347256376410398924e1_f64) * t105203 * t7654;
    (t105193, t105202, t105206)
}
