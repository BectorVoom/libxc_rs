//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2247/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2247(t1243: f64, t29109: f64, t105121: f64, t73: f64, t1032: f64, t5412: f64, t2148: f64, t1214: f64, t1248: f64, t12657: f64, t1287: f64, t17968: f64, t1829: f64, t26889: f64, t26895: f64, t26922: f64, t26949: f64, t26963: f64, t26999: f64, t29118: f64, t29159: f64, t29167: f64, t29271: f64, t29275: f64, t3584: f64, t3790: f64, t5237: f64, t5458: f64, t7632: f64, t7635: f64, t7636: f64, t7637: f64, t7643: f64, t7652: f64, t7654: f64, t7659: f64, t8197: f64, t8198: f64, t8201: f64, t8208: f64, t97422: f64, t97425: f64) -> (f64, f64, f64) {
    let t105167 = t1243 * t29109;
    let t105193 = t105121 * t73;
    let t105202 = t5412 * t1032;
    let t105203 = t2148 * t105202;
    let t105206 = 0.17347256376410398924e1_f64 * t26922 * t29271 * t1248 * t1287 - 0.39512695097613069591e1_f64 * t7632 * t17968 + 0.17347256376410398924e1_f64 * t7636 * t7652 * t8197 * t3790 - 0.8673628188205199462e0_f64 * t7659 * t105167 * t1248 * t1287 - 0.17347256376410398924e1_f64 * t7643 * t7652 * t8208 * t3584 - 0.8673628188205199462e0_f64 * t12657 * t7635 * t8198 + 0.13170898365871023197e1_f64 * t26999 * t5237 - 0.52041769129231196772e1_f64 * t26949 * t7637 * t29118 * t1214 - 0.26020884564615598386e1_f64 * t26949 * t7637 * t8201 * t3584 - 0.65854491829355115987e0_f64 * t97425 * t1829 + 0.17347256376410398924e1_f64 * t97422 * t29167 - 0.17347256376410398924e1_f64 * t26889 * t105193 * t29159 + 0.17347256376410398924e1_f64 * t26895 * t105193 * t5458 - 0.8673628188205199462e0_f64 * t29275 * t26963 + 0.17347256376410398924e1_f64 * t105203 * t7654;
    (t105193, t105202, t105206)
}
