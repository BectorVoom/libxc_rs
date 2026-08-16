//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1421;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1422;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1423;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1424;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1425;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1426;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1427;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1428;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta394(t1216: f64, t5971: f64, t11668: f64, t1090: f64, t6225: f64, t3578: f64, t11697: f64, t6191: f64, t3577: f64, t248: f64, t3570: f64, t6219: f64, t1213: f64, t5979: f64, t5975: f64, t11678: f64, t11709: f64, t11734: f64, t1227: f64, t15438: f64, t15569: f64, t18342: f64, t18346: f64, t18357: f64, t18360: f64, t3490: f64, t4954: f64, t4984: f64, t5014: f64, t5019: f64, t6203: f64, t6227: f64, t6232: f64, t3521: f64, t1409: f64, t15701: f64, t15700: f64, t1735: f64, t4729: f64, t18232: f64, t4900: f64, t3450: f64, t5398: f64, t3449: f64, t18237: f64, t4908: f64, t3448: f64, t6138: f64, t3451: f64, t6144: f64, t18225: f64, t11583: f64, t5392: f64, t18221: f64, t15320: f64, t4904: f64, t15313: f64, t4919: f64, t11531: f64, t15265: f64, t15376: f64, t3447: f64, t4901: f64, t15395: f64, t18206: f64, t15338: f64, t3431: f64, t6126: f64, t1174: f64, t6130: f64, t11539: f64, t6119: f64, t4889: f64, t4896: f64, t18215: f64, t11570: f64, t11569: f64, t1180: f64, t15284: f64, t15287: f64, t15300: f64, t15307: f64, t18321: f64, t4937: f64, t18211: f64, t15382: f64, t15390: f64, t1171: f64, t6109: f64, t6011: f64, t699: f64, t11219: f64, t136: f64, t3297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18364, t18368, t18372, t18375) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1421(t1216, t5971, t11668, t1090, t6225, t3578, t11697, t6191, t3577, t248, t3570, t6219);
        let t18390 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1422(t1213, t18375, t1216, t5979, t3578, t5975, t11678, t11709, t11734, t1227, t15438, t15569, t18342, t18346, t18357, t18360, t18364, t18368, t18372, t3490, t3577, t4954, t4984, t5014, t5019, t6203, t6227, t6232);
        let (t18393, t18397, t18401, t18404) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1423(t248, t3521, t5975, t1227, t1409, t15701, t15700, t3578, t1735, t4729, t18232, t4900);
        let (t18410, t18413, t18417, t18421, t18424, t18427) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1424(t3450, t5398, t3449, t18237, t4908, t3448, t6138, t3451, t6144, t18225, t11583, t5392);
        let t18442 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1425(t18427, t3449, t18221, t4908, t15320, t4904, t15313, t4919, t11531, t15265, t15376, t18404, t18410, t18413, t18417, t18421, t18424, t3447, t4901);
        let (t18443, t18447, t18452, t18455, t18458, t18460) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1426(t15395, t18206, t15338, t4904, t3447, t3431, t6126, t1174, t6130, t11539, t6119, t4889, t4896);
        let t18473 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1427(t18215, t4900, t11570, t5392, t11569, t1180, t15284, t15287, t15300, t15307, t18321, t18443, t18447, t18452, t18455, t18458, t18460, t3447, t4889, t4937);
        let (t18475, t18484, t18489, t18494, t18497, t18499) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1428(t18211, t4900, t15382, t15390, t1171, t6109, t6011, t699, t11219, t18206, t136, t3297);
    (t18390, t18393, t18397, t18401, t18442, t18473, t18475, t18484, t18489, t18494, t18497, t18499)
}
