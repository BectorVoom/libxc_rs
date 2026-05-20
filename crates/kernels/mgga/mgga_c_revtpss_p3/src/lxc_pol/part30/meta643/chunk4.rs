//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2253/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2253<F: Float>(t1214: F, t1769: F, t1248: F, t73: F, t8190: F, t1209: F, t29109: F, t1215: F, t1287: F, t1294: F, t18084: F, t1829: F, t2151: F, t26889: F, t26895: F, t26928: F, t26937: F, t26979: F, t26984: F, t26994: F, t27025: F, t29159: F, t29186: F, t29187: F, t29224: F, t29271: F, t29275: F, t29279: F, t29283: F, t5458: F, t7602: F, t7637: F, t7643: F, t7652: F, t8217: F, t96933: F, t97066: F) -> F {
    let t105460 = t1769 * t1214;
    let t105490 = t8190 * t1248 * t73;
    let t105499 = t1209 * t29109;
    let t105504 = -F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t105460 * t1294 + F::cast_from(0.17347256376410398924e1_f64) * t26937 * t29283 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29271 * t1214 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t29186 * t1248 * t1287 - F::cast_from(0.17347256376410398924e1_f64) * t27025 * t29187 + F::cast_from(0.34694512752820797848e1_f64) * t26994 * t7637 * t29186 * t1214 + F::cast_from(0.34694512752820797848e1_f64) * t29275 * t26928 + F::cast_from(0.17347256376410398924e1_f64) * t26979 * t29279 + F::cast_from(0.34694512752820797848e1_f64) * t27025 * t29224 + F::cast_from(0.65854491829355115987e0_f64) * t7602 * t18084 - F::cast_from(0.17347256376410398924e1_f64) * t26889 * t105490 * t29159 + F::cast_from(0.17347256376410398924e1_f64) * t26895 * t105490 * t5458 - F::cast_from(0.13170898365871023197e1_f64) * t96933 * t1829 - F::cast_from(0.13170898365871023197e1_f64) * t105499 * t1215 - F::cast_from(0.8673628188205199462e0_f64) * t26984 * t8217;
    t105504
}
