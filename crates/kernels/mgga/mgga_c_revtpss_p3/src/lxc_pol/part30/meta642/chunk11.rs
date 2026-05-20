//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2248/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2248<F: Float>(t1276: F, t2148: F, t3140: F, t5412: F, t1203: F, t1828: F, t1214: F, t104529: F, t105193: F, t2151: F, t26913: F, t26922: F, t26936: F, t26959: F, t26979: F, t27005: F, t27008: F, t27011: F, t27025: F, t29109: F, t29136: F, t29158: F, t29166: F, t29195: F, t29233: F, t29271: F, t29301: F, t3555: F, t3584: F, t3721: F, t5246: F, t5429: F, t5457: F, t5464: F, t7636: F, t7637: F, t7643: F, t7652: F, t7662: F, t8190: F, t8198: F, t8205: F, t96986: F, t97066: F, t97304: F) -> F {
    let t105220 = t2148 * t5412 * t3140 * t1276;
    let t105236 = t1828 * t1203;
    let t105241 = t1203 * t1214;
    let t105258 = F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29271 * t1203 - F::cast_from(0.4336814094102599731e0_f64) * t8205 * t27005 - F::cast_from(0.17347256376410398924e1_f64) * t3555 * t26936 * t8198 + F::cast_from(0.8673628188205199462e0_f64) * t29136 * t26959 - F::cast_from(0.8673628188205199462e0_f64) * t105220 * t7662 - F::cast_from(0.13170898365871023197e1_f64) * t27011 * t5246 + F::cast_from(0.17347256376410398924e1_f64) * t26922 * t105193 * t29166 + F::cast_from(0.34694512752820797848e1_f64) * t96986 * t29195 * t5464 * t3721 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t7637 * t8190 * t3584 - F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t105236 * t1214 + F::cast_from(0.34694512752820797848e1_f64) * t97304 * t29158 * t5457 * t105241 + F::cast_from(0.17347256376410398924e1_f64) * t26979 * t29233 + F::cast_from(0.26341796731742046394e1_f64) * t27008 * t5429 + F::cast_from(0.4336814094102599731e0_f64) * t104529 * t26913 - F::cast_from(0.17347256376410398924e1_f64) * t7636 * t7637 * t29109 * t1203 - F::cast_from(0.17347256376410398924e1_f64) * t27025 * t29301;
    t105258
}
