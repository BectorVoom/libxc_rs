//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 834/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk834<F: Float>(t1274: F, t403: F, t4537: F, t1228: F, t1230: F, t1232: F, t1253: F, t1257: F, t1259: F, t1261: F, t1304: F, t1314: F, t1323: F, t18552: F, t18563: F, t18638: F, t18639: F, t18642: F, t18779: F, t18850: F, t18853: F, t18854: F, t18863: F, t18865: F, t18885: F, t18886: F, t18889: F, t440: F, t441: F, t450: F, t456: F, t4606: F, t4673: F, t4679: F, t4681: F, t4737: F, t62: F, t75: F) -> (F, F) {
    let t18899 = 8.0 * t1274 * t4537 * t403;
    let t18910 = 0.58482233974552040708e0 * t450 * t18552 * t456 + 0.91080982599109921211e5 * t75 * t18638 * t18639 * t18642 - t18850 - t18853 + 0.96494049533612093922e2 * t1259 * t18854 * t1261 - 0.35089340384731224426e1 * t1304 * t18563 * t456 + t18863 - 0.12304676425209353917e5 * t75 * t18865 * t18639 * t4737 - 0.24829604254387158296e5 * t62 / t1257 / t1228 * t18779 * t4681 - 6.0 * t1230 * t18854 * t441 + 0.11579285944033451271e4 * t4679 * t18779 * t1261 - t18885 + 0.19965908856856833625e6 * t62 / t18886 * t18779 / t18889 + 36.0 * t1259 * t1232 * t1253 + t18899 - 0.1403573615389248977e2 * t4606 * t18639 * t456 + 0.1286587327114827919e3 * t1259 * t4673 * t1261 * t440 - 0.62336721237753107879e3 * t4606 * t1323 * t1314;
    (t18899, t18910)
}
