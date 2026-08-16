//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta627 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2174;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2175;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2176;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2177;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2178;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2179;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2180;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2181;
use chunk8::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2182;
use chunk9::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2183;
use chunk10::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2184;
use chunk11::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2185;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta627<F: Float>(t25375: F, t99365: F, t1579: F, t25392: F, t4481: F, t92921: F, t10073: F, t1958: F, t25390: F, t25305: F, t99380: F, t213: F, t27265: F, t2453: F, t2458: F, t7760: F, t25326: F, t25394: F, t27199: F, t887: F, t93306: F, t93312: F, t93315: F, t93318: F, t93322: F, t93324: F, t93349: F, t25331: F, t27213: F, t93190: F, t99211: F, t25374: F, t98848: F, t25378: F, t99403: F, t231: F, t2645: F, t7070: F, t7076: F, t7759: F, t836: F, t93326: F, t93331: F, t93334: F, t93335: F, t93337: F, t93339: F, t93343: F, t93346: F, t93365: F, t99161: F, t1580: F, t25338: F, t689: F, t25365: F, t27279: F, t7058: F, t99201: F, t99349: F, t14983: F, t25399: F, t7064: F, t99321: F, t25411: F, t99389: F, t93369: F, t93372: F, t93375: F, t93378: F, t93382: F, t93384: F, t2435: F, t7774: F, t25431: F, t14481: F, t1950: F, t2782: F, t4424: F, t886: F, t2439: F, t780: F, t785: F, t14495: F, t14979: F, t25391: F, t27189: F, t27353: F, t27357: F, t27358: F, t2772: F, t51608: F, t7053: F, t7071: F, t7073: F, t92864: F, t93387: F, t93389: F, t93391: F, t99237: F, t99303: F, t98831: F, t98864: F, t98895: F, t98932: F, t99159: F, t99194: F, t99227: F, t99264: F, t99295: F, t99332: F, t99368: F, t99409: F, t892: F, t1940: F, t1963: F, t580: F, t4343: F, t605: F, t27383: F, t63164: F, t2411: F, t27363: F, t25207: F, t61102: F, t2403: F, t25206: F, t25215: F, t25436: F, t25440: F, t27376: F, t27382: F, t27391: F, t27402: F, t30: F, t4541: F, t7092: F, t7749: F, t7783: F, t92819: F, t98780: F, t98784: F, t98787: F, t98793: F, t98678: F, t98725: F, t98776: F, t14365: F, t14436: F, t14468: F, t14749: F, t14767: F, t198: F, t207: F, t2394: F, t2408: F, t25445: F, t27368: F, t27384: F, t4433: F, t61155: F, t61182: F, t7087: F, t7091: F, t92742: F, t93404: F, t98722: F, t98759: F, t98779: F, t98786: F, t15071: F, t1544: F, t1583: F, t18875: F, t2430: F, t27158: F, t27364: F, t27375: F, t2832: F, t4537: F, t51780: F, t61203: F, t63186: F, t775: F, t7847: F, t890: F, t92775: F, t98651: F) -> (F, F, F, F, F, F) {
        let (t99412, t99414, t99420, t99423, t99425, t99429) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2174::<F>(t25375, t99365, t1579, t25392, t4481, t92921, t10073, t1958, t25390, t25305, t99380, t213, t27265);
        let t99440 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2175::<F>(t2453, t2458, t7760, t25326, t25394, t27199, t887, t93306, t93312, t93315, t93318, t93322, t93324, t93349, t99412, t99414, t99420, t99423, t99425, t99429);
        let t99469 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2176::<F>(t25331, t27213, t93190, t99211, t25374, t98848, t25378, t99403, t231, t2645, t27265, t7070, t7076, t7759, t836, t93326, t93331, t93334, t93335, t93337, t93339, t93343, t93346, t93365);
        let (t99472, t99475, t99480, t99481, t99485, t99487) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2177::<F>(t25375, t99161, t1580, t25338, t689, t25365, t27279, t7058, t99201, t99349, t14983, t25399);
        let t99494 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2178::<F>(t7064, t99321, t25411, t99389, t93369, t93372, t93375, t93378, t93382, t93384, t99472, t99475, t99480, t99481, t99485, t99487);
        let (t99495, t99496, t99502, t99512, t99520) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2179::<F>(t2435, t7774, t25431, t14481, t1950, t2782, t4424, t886, t2439, t7759, t780, t785);
        let t99532 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2180::<F>(t25411, t99495, t14495, t14979, t25391, t25392, t27189, t27265, t27353, t27357, t27358, t2772, t51608, t7053, t7070, t7071, t7073, t886, t92864, t93387, t93389, t93391, t99237, t99303, t99496, t99502, t99512, t99520);
        let t99536 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2181::<F>(t98831, t98864, t98895, t98932, t99159, t99194, t99227, t99264, t99295, t99332, t99368, t99409, t99440, t99469, t99494, t99532);
        let (t99537, t99542, t99543, t99550, t99555, t99558) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2182::<F>(t892, t99536, t1940, t1963, t580, t4343, t605, t27383, t63164, t2411, t27363, t25207, t61102);
        let t99563 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2183::<F>(t1940, t1963, t2403, t25206, t25215, t25436, t25440, t27376, t27382, t27391, t27402, t30, t4541, t7092, t7749, t7783, t92819, t98780, t98784, t98787, t98793, t99537, t99542, t99543, t99550, t99555, t99558);
        let (t99565, t100882) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2184::<F>(t98678, t98725, t98776, t99563, t14365, t14436, t14468, t14749, t14767, t1940, t1963, t198, t207, t2394, t2403, t2408, t25206, t25445, t27368, t27384, t4433, t4541, t61155, t61182, t63164, t7087, t7091, t7783, t892, t92742, t93404, t98722, t98759, t98779, t98786, t99536);
        let t100926 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2185::<F>(t15071, t1544, t1583, t18875, t1940, t2403, t2430, t25436, t25440, t27158, t27364, t27368, t27375, t2832, t4343, t4537, t51780, t61102, t61203, t63186, t7087, t7091, t775, t7783, t7847, t890, t92775, t98651, t99555);
    (t99537, t99542, t99555, t99565, t100882, t100926)
}
