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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta621<F: Float>(t25375: F, t99365: F, t1579: F, t25392: F, t4481: F, t92921: F, t10073: F, t1958: F, t25390: F, t25305: F, t99380: F, t213: F, t27265: F, t2453: F, t2458: F, t7760: F, t25326: F, t25394: F, t27199: F, t887: F, t93306: F, t93312: F, t93315: F, t93318: F, t93322: F, t93324: F, t93349: F, t25331: F, t27213: F, t93190: F, t99211: F, t25374: F, t98848: F, t25378: F, t99403: F, t231: F, t2645: F, t7070: F, t7076: F, t7759: F, t836: F, t93326: F, t93331: F, t93334: F, t93335: F, t93337: F, t93339: F, t93343: F, t93346: F, t93365: F, t99161: F, t1580: F, t25338: F, t689: F, t25365: F, t27279: F, t7058: F, t99201: F, t99349: F, t14983: F, t25399: F, t7064: F, t99321: F, t25411: F, t99389: F, t93369: F, t93372: F, t93375: F, t93378: F, t93382: F, t93384: F, t2435: F, t7774: F, t25431: F, t14481: F, t1950: F, t2782: F, t4424: F, t886: F, t2439: F, t780: F, t785: F, t14495: F, t14979: F, t25391: F, t27189: F, t27353: F, t27357: F, t27358: F, t2772: F, t51608: F, t7053: F, t7071: F, t7073: F, t92864: F, t93387: F, t93389: F, t93391: F, t99237: F, t99303: F, t98831: F, t98864: F, t98895: F, t98932: F, t99159: F, t99194: F, t99227: F, t99264: F, t99295: F, t99332: F, t99368: F, t99409: F, t892: F, t1940: F, t1963: F, t580: F, t4343: F, t605: F, t27383: F, t63164: F, t2411: F, t27363: F, t25207: F, t61102: F, t2403: F, t25206: F, t25215: F, t25436: F, t25440: F, t27376: F, t27382: F, t27391: F, t27402: F, t30: F, t4541: F, t7092: F, t7749: F, t7783: F, t92819: F, t98780: F, t98784: F, t98787: F, t98793: F, t98678: F, t98725: F, t98776: F, t1096: F, t357: F, t1043: F, t1089: F, t16318: F, t16577: F, t25473: F, t25611: F, t25648: F, t25695: F, t27543: F, t27556: F, t27575: F, t27642: F, t27651: F, t27661: F, t27696: F, t3075: F, t3118: F, t3270: F, t3325: F, t4764: F, t4975: F, t4982: F, t4997: F, t7140: F, t7145: F, t7151: F, t7159: F, t7160: F, t7810: F, t93497: F, t93498: F, t94016: F, t94063: F, t94080: F, t94085: F, t94122: F, t988: F, t999: F, t1976: F, t4743: F, t1695: F, t342: F, t1097: F, t15579: F, t16328: F, t1985: F, t25605: F, t25629: F, t25699: F, t27411: F, t27440: F, t27444: F, t27679: F, t27691: F, t3059: F, t3066: F, t4941: F, t5015: F, t7102: F, t7135: F, t7144: F, t7822: F, t93485: F, t93921: F) -> (F, F, F, F, F, F, F) {
        let (t99412, t99414, t99420, t99423, t99425, t99429) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2189::<F>(t25375, t99365, t1579, t25392, t4481, t92921, t10073, t1958, t25390, t25305, t99380, t213, t27265);
        let t99440 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2190::<F>(t2453, t2458, t7760, t25326, t25394, t27199, t887, t93306, t93312, t93315, t93318, t93322, t93324, t93349, t99412, t99414, t99420, t99423, t99425, t99429);
        let t99469 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2191::<F>(t25331, t27213, t93190, t99211, t25374, t98848, t25378, t99403, t231, t2645, t27265, t7070, t7076, t7759, t836, t93326, t93331, t93334, t93335, t93337, t93339, t93343, t93346, t93365);
        let (t99472, t99475, t99480, t99481, t99485, t99487) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2192::<F>(t25375, t99161, t1580, t25338, t689, t25365, t27279, t7058, t99201, t99349, t14983, t25399);
        let t99494 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2193::<F>(t7064, t99321, t25411, t99389, t93369, t93372, t93375, t93378, t93382, t93384, t99472, t99475, t99480, t99481, t99485, t99487);
        let (t99495, t99496, t99502, t99512, t99520) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2194::<F>(t2435, t7774, t25431, t14481, t1950, t2782, t4424, t886, t2439, t7759, t780, t785);
        let t99532 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2195::<F>(t25411, t99495, t14495, t14979, t25391, t25392, t27189, t27265, t27353, t27357, t27358, t2772, t51608, t7053, t7070, t7071, t7073, t886, t92864, t93387, t93389, t93391, t99237, t99303, t99496, t99502, t99512, t99520);
        let t99536 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2196::<F>(t98831, t98864, t98895, t98932, t99159, t99194, t99227, t99264, t99295, t99332, t99368, t99409, t99440, t99469, t99494, t99532);
        let (t99537, t99542, t99543, t99550, t99555, t99558) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2197::<F>(t892, t99536, t1940, t1963, t580, t4343, t605, t27383, t63164, t2411, t27363, t25207, t61102);
        let t99563 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2198::<F>(t1940, t1963, t2403, t25206, t25215, t25436, t25440, t27376, t27382, t27391, t27402, t30, t4541, t7092, t7749, t7783, t92819, t98780, t98784, t98787, t98793, t99537, t99542, t99543, t99550, t99555, t99558);
        let (t99565, t99618) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2199::<F>(t98678, t98725, t98776, t99563, t1096, t357, t1043, t1089, t16318, t16577, t25473, t25611, t25648, t25695, t27543, t27556, t27575, t27642, t27651, t27661, t27696, t3075, t3118, t3270, t3325, t4764, t4975, t4982, t4997, t7140, t7145, t7151, t7159, t7160, t7810, t93497, t93498, t94016, t94063, t94080, t94085, t94122, t988, t999);
        let t99673 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2200::<F>(t1976, t4743, t1695, t988, t27543, t342, t1043, t1089, t1096, t1097, t15579, t16328, t1985, t25605, t25629, t25695, t25699, t27411, t27440, t27444, t27651, t27679, t27691, t3059, t3066, t4941, t4975, t5015, t7102, t7135, t7144, t7145, t7159, t7160, t7810, t7822, t93485, t93497, t93498, t93921, t94122, t999);
    (t99536, t99537, t99542, t99555, t99565, t99618, t99673)
}
