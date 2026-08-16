//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 414/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk414<F: Float>(t2131: F, t231: F, t1216: F, t725: F, t1227: F, t728: F, t1238: F, t1246: F, t1179: F, t1184: F, t1191: F, t1206: F, t1214: F, t2042: F) -> (F, F, F, F, F, F) {
    let t2132 = t231 * t2131;
    let t2134 = F::cast_from(0.24415406715670879921e-3_f64) * t725 * t1216;
    let t2136 = F::cast_from(0.11696446794910408142e1_f64) * t728 * t1227;
    let t2138 = F::cast_from(0.58482233974552040708e0_f64) * t728 * t1238;
    let t2140 = F::cast_from(0.17315755899375863299e2_f64) * t728 * t1246;
    let t2141 = -t1179 - t1184 - t1191 + t1206 + t1214 + t2132 + t2134 + t2042 + t2136 - t2138 - t2140;
    (t2132, t2134, t2136, t2138, t2140, t2141)
}
