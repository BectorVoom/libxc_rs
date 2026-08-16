//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2253/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2253(t1214: f64, t1769: f64, t1248: f64, t73: f64, t8190: f64, t1209: f64, t29109: f64, t1215: f64, t1287: f64, t1294: f64, t18084: f64, t1829: f64, t2151: f64, t26889: f64, t26895: f64, t26928: f64, t26937: f64, t26979: f64, t26984: f64, t26994: f64, t27025: f64, t29159: f64, t29186: f64, t29187: f64, t29224: f64, t29271: f64, t29275: f64, t29279: f64, t29283: f64, t5458: f64, t7602: f64, t7637: f64, t7643: f64, t7652: f64, t8217: f64, t96933: f64, t97066: f64) -> f64 {
    let t105460 = t1769 * t1214;
    let t105490 = t8190 * t1248 * t73;
    let t105499 = t1209 * t29109;
    let t105504 = -0.69389025505641595696e1_f64 * t97066 * t2151 * t105460 * t1294 + 0.17347256376410398924e1_f64 * t26937 * t29283 - 0.34694512752820797848e1_f64 * t7643 * t7652 * t29271 * t1214 - 0.17347256376410398924e1_f64 * t26889 * t29186 * t1248 * t1287 - 0.17347256376410398924e1_f64 * t27025 * t29187 + 0.34694512752820797848e1_f64 * t26994 * t7637 * t29186 * t1214 + 0.34694512752820797848e1_f64 * t29275 * t26928 + 0.17347256376410398924e1_f64 * t26979 * t29279 + 0.34694512752820797848e1_f64 * t27025 * t29224 + 0.65854491829355115987e0_f64 * t7602 * t18084 - 0.17347256376410398924e1_f64 * t26889 * t105490 * t29159 + 0.17347256376410398924e1_f64 * t26895 * t105490 * t5458 - 0.13170898365871023197e1_f64 * t96933 * t1829 - 0.13170898365871023197e1_f64 * t105499 * t1215 - 0.8673628188205199462e0_f64 * t26984 * t8217;
    t105504
}
