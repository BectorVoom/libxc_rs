//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2189;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2190;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2191;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2192;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2193;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2194;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2195;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2196;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2197;
use chunk9::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2198;
use chunk10::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2199;
use chunk11::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta621(t25375: f64, t99365: f64, t1579: f64, t25392: f64, t4481: f64, t92921: f64, t10073: f64, t1958: f64, t25390: f64, t25305: f64, t99380: f64, t213: f64, t27265: f64, t2453: f64, t2458: f64, t7760: f64, t25326: f64, t25394: f64, t27199: f64, t887: f64, t93306: f64, t93312: f64, t93315: f64, t93318: f64, t93322: f64, t93324: f64, t93349: f64, t25331: f64, t27213: f64, t93190: f64, t99211: f64, t25374: f64, t98848: f64, t25378: f64, t99403: f64, t231: f64, t2645: f64, t7070: f64, t7076: f64, t7759: f64, t836: f64, t93326: f64, t93331: f64, t93334: f64, t93335: f64, t93337: f64, t93339: f64, t93343: f64, t93346: f64, t93365: f64, t99161: f64, t1580: f64, t25338: f64, t689: f64, t25365: f64, t27279: f64, t7058: f64, t99201: f64, t99349: f64, t14983: f64, t25399: f64, t7064: f64, t99321: f64, t25411: f64, t99389: f64, t93369: f64, t93372: f64, t93375: f64, t93378: f64, t93382: f64, t93384: f64, t2435: f64, t7774: f64, t25431: f64, t14481: f64, t1950: f64, t2782: f64, t4424: f64, t886: f64, t2439: f64, t780: f64, t785: f64, t14495: f64, t14979: f64, t25391: f64, t27189: f64, t27353: f64, t27357: f64, t27358: f64, t2772: f64, t51608: f64, t7053: f64, t7071: f64, t7073: f64, t92864: f64, t93387: f64, t93389: f64, t93391: f64, t99237: f64, t99303: f64, t98831: f64, t98864: f64, t98895: f64, t98932: f64, t99159: f64, t99194: f64, t99227: f64, t99264: f64, t99295: f64, t99332: f64, t99368: f64, t99409: f64, t892: f64, t1940: f64, t1963: f64, t580: f64, t4343: f64, t605: f64, t27383: f64, t63164: f64, t2411: f64, t27363: f64, t25207: f64, t61102: f64, t2403: f64, t25206: f64, t25215: f64, t25436: f64, t25440: f64, t27376: f64, t27382: f64, t27391: f64, t27402: f64, t30: f64, t4541: f64, t7092: f64, t7749: f64, t7783: f64, t92819: f64, t98780: f64, t98784: f64, t98787: f64, t98793: f64, t98678: f64, t98725: f64, t98776: f64, t1096: f64, t357: f64, t1043: f64, t1089: f64, t16318: f64, t16577: f64, t25473: f64, t25611: f64, t25648: f64, t25695: f64, t27543: f64, t27556: f64, t27575: f64, t27642: f64, t27651: f64, t27661: f64, t27696: f64, t3075: f64, t3118: f64, t3270: f64, t3325: f64, t4764: f64, t4975: f64, t4982: f64, t4997: f64, t7140: f64, t7145: f64, t7151: f64, t7159: f64, t7160: f64, t7810: f64, t93497: f64, t93498: f64, t94016: f64, t94063: f64, t94080: f64, t94085: f64, t94122: f64, t988: f64, t999: f64, t1976: f64, t4743: f64, t1695: f64, t342: f64, t1097: f64, t15579: f64, t16328: f64, t1985: f64, t25605: f64, t25629: f64, t25699: f64, t27411: f64, t27440: f64, t27444: f64, t27679: f64, t27691: f64, t3059: f64, t3066: f64, t4941: f64, t5015: f64, t7102: f64, t7135: f64, t7144: f64, t7822: f64, t93485: f64, t93921: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t99412, t99414, t99420, t99423, t99425, t99429) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2189(t25375, t99365, t1579, t25392, t4481, t92921, t10073, t1958, t25390, t25305, t99380, t213, t27265);
        let t99440 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2190(t2453, t2458, t7760, t25326, t25394, t27199, t887, t93306, t93312, t93315, t93318, t93322, t93324, t93349, t99412, t99414, t99420, t99423, t99425, t99429);
        let t99469 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2191(t25331, t27213, t93190, t99211, t25374, t98848, t25378, t99403, t231, t2645, t27265, t7070, t7076, t7759, t836, t93326, t93331, t93334, t93335, t93337, t93339, t93343, t93346, t93365);
        let (t99472, t99475, t99480, t99481, t99485, t99487) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2192(t25375, t99161, t1580, t25338, t689, t25365, t27279, t7058, t99201, t99349, t14983, t25399);
        let t99494 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2193(t7064, t99321, t25411, t99389, t93369, t93372, t93375, t93378, t93382, t93384, t99472, t99475, t99480, t99481, t99485, t99487);
        let (t99495, t99496, t99502, t99512, t99520) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2194(t2435, t7774, t25431, t14481, t1950, t2782, t4424, t886, t2439, t7759, t780, t785);
        let t99532 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2195(t25411, t99495, t14495, t14979, t25391, t25392, t27189, t27265, t27353, t27357, t27358, t2772, t51608, t7053, t7070, t7071, t7073, t886, t92864, t93387, t93389, t93391, t99237, t99303, t99496, t99502, t99512, t99520);
        let t99536 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2196(t98831, t98864, t98895, t98932, t99159, t99194, t99227, t99264, t99295, t99332, t99368, t99409, t99440, t99469, t99494, t99532);
        let (t99537, t99542, t99543, t99550, t99555, t99558) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2197(t892, t99536, t1940, t1963, t580, t4343, t605, t27383, t63164, t2411, t27363, t25207, t61102);
        let t99563 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2198(t1940, t1963, t2403, t25206, t25215, t25436, t25440, t27376, t27382, t27391, t27402, t30, t4541, t7092, t7749, t7783, t92819, t98780, t98784, t98787, t98793, t99537, t99542, t99543, t99550, t99555, t99558);
        let (t99565, t99618) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2199(t98678, t98725, t98776, t99563, t1096, t357, t1043, t1089, t16318, t16577, t25473, t25611, t25648, t25695, t27543, t27556, t27575, t27642, t27651, t27661, t27696, t3075, t3118, t3270, t3325, t4764, t4975, t4982, t4997, t7140, t7145, t7151, t7159, t7160, t7810, t93497, t93498, t94016, t94063, t94080, t94085, t94122, t988, t999);
        let t99673 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2200(t1976, t4743, t1695, t988, t27543, t342, t1043, t1089, t1096, t1097, t15579, t16328, t1985, t25605, t25629, t25695, t25699, t27411, t27440, t27444, t27651, t27679, t27691, t3059, t3066, t4941, t4975, t5015, t7102, t7135, t7144, t7145, t7159, t7160, t7810, t7822, t93485, t93497, t93498, t93921, t94122, t999);
    (t99536, t99537, t99542, t99555, t99565, t99618, t99673)
}
