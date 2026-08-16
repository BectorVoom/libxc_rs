//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 918/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk918(t1257: f64, t1260: f64, t1274: f64, t403: f64, t4537: f64, t1228: f64, t1230: f64, t1232: f64, t1253: f64, t1259: f64, t1261: f64, t1304: f64, t1314: f64, t1323: f64, t18552: f64, t18563: f64, t18638: f64, t18639: f64, t18642: f64, t18779: f64, t18850: f64, t18853: f64, t18854: f64, t18863: f64, t18865: f64, t18885: f64, t440: f64, t441: f64, t450: f64, t456: f64, t4606: f64, t4673: f64, t4679: f64, t4681: f64, t4737: f64, t62: f64, t75: f64) -> (f64, f64) {
    let t18886 = t1257 * t1257;
    let t18889 = t1260 * t1260;
    let t18899 = 8.0_f64 * t1274 * t4537 * t403;
    let t18910 = 0.58482233974552040708e0_f64 * t450 * t18552 * t456 + 0.91080982599109921211e5_f64 * t75 * t18638 * t18639 * t18642 - t18850 - t18853 + 0.96494049533612093922e2_f64 * t1259 * t18854 * t1261 - 0.35089340384731224426e1_f64 * t1304 * t18563 * t456 + t18863 - 0.12304676425209353917e5_f64 * t75 * t18865 * t18639 * t4737 - 0.24829604254387158296e5_f64 * t62 / t1257 / t1228 * t18779 * t4681 - 6.0_f64 * t1230 * t18854 * t441 + 0.11579285944033451271e4_f64 * t4679 * t18779 * t1261 - t18885 + 0.19965908856856833625e6_f64 * t62 / t18886 * t18779 / t18889 + 36.0_f64 * t1259 * t1232 * t1253 + t18899 - 0.1403573615389248977e2_f64 * t4606 * t18639 * t456 + 0.1286587327114827919e3_f64 * t1259 * t4673 * t1261 * t440 - 0.62336721237753107879e3_f64 * t4606 * t1323 * t1314;
    (t18899, t18910)
}
