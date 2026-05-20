//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1728;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1729;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1730;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1731;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1732;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1733;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1734;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta472<F: Float>(t1903: F, t5774: F, t4076: F, t6918: F, t72: F, t686: F, t3915: F, t6889: F, t786: F, t1364: F, t14100: F, t5722: F, t1357: F, t6919: F, t689: F, t1444: F, t14081: F, t14084: F, t14087: F, t1424: F, t14299: F, t1904: F, t9677: F, t9687: F, t9691: F, t5599: F, t10157: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t14276: F, t5715: F, t5728: F, t9694: F, t9695: F, t6895: F, t9657: F, t22307: F, t225: F, t212: F, t6888: F, t1358: F, t6896: F, t9680: F, t10160: F, t10163: F, t10166: F, t14280: F, t14290: F, t14294: F, t14297: F, t213: F, t4071: F, t561: F, t22393: F, t1343: F, t1353: F, t13599: F, t13600: F, t1450: F, t1868: F, t198: F, t21901: F, t21905: F, t21933: F, t21937: F, t21969: F, t4139: F, t532: F, t5532: F, t5536: F, t5591: F, t5627: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F, t4147: F, t6781: F, t4140: F, t6836: F, t13615: F, t13620: F, t13623: F, t13634: F, t13635: F, t22187: F, t22189: F, t22192: F, t22194: F, t22196: F, t22197: F, t22198: F, t22199: F, t22200: F, t22201: F, t22202: F, t9394: F, t9415: F, t9593: F, t5537: F, t13643: F, t1448: F, t22205: F, t22206: F, t22207: F, t22208: F, t22209: F, t22211: F, t5541: F, t9421: F, t9427: F, t9429: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F, t9588: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22395, t22399, t22400, t22405, t22407) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1728::<F>(t1903, t5774, t4076, t6918, t72, t686, t3915, t6889, t786, t1364, t14100, t5722);
        let (t22415, t22418) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1729::<F>(t1357, t6919, t689, t1444, t6918, t4076, t14081, t14084, t14087, t1424, t14299, t1904, t22395, t22400, t22405, t22407, t9677, t9687, t9691);
        let t22430 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1730::<F>(t1904, t5599, t689, t10157, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14276, t5715, t5728, t9694, t9695);
        let (t22433, t22441, t22447, t22450, t22452) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1731::<F>(t1444, t6895, t9657, t22307, t225, t212, t6888, t1358, t689, t1357, t6896, t72);
        let (t22453, t22459) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1732::<F>(t22452, t686, t9680, t10160, t10163, t10166, t1424, t14280, t14290, t14294, t14297, t213, t22433, t22441, t22447, t22450, t4071, t561, t6919);
        let (t22461, t22465) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1733::<F>(t22393, t22418, t22430, t22459, t1343, t1353, t13599, t13600, t1450, t1868, t198, t21901, t21905, t21933, t21937, t21969, t4139, t532, t5532, t5536, t5591, t5627, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9374, t9389, t9391);
        let t22473 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1734::<F>(t4147, t6781, t4140, t6836, t1353, t13615, t13620, t13623, t13634, t13635, t22187, t22189, t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t22202, t4139, t5536, t9394, t9415);
        let (t22475, t22482) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1735::<F>(t6781, t9593, t5537, t5591, t13643, t1448, t22205, t22206, t22207, t22208, t22209, t22211, t5536, t5541, t9421, t9427, t9429, t9514, t9517, t9521, t9546, t9569, t9574, t9577, t9588);
    (t22395, t22399, t22415, t22433, t22453, t22461, t22465, t22473, t22475, t22482)
}
