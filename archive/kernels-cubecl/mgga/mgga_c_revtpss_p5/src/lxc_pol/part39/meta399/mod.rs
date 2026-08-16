//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta399 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1449;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1450;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1451;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1452;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1453;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1454;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1455;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1456;
use chunk8::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1457;
use chunk9::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1458;
use chunk10::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1459;
use chunk11::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1460;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta399<F: Float>(t1269: F, t1287: F, t5284: F, t17633: F, t5458: F, t17482: F, t3769: F, t3783: F, t12713: F, t5332: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t3601: F, t17710: F, t13141: F, t3603: F, t1234: F, t12717: F, t12751: F, t12756: F, t1285: F, t12966: F, t12975: F, t17188: F, t17192: F, t17808: F, t17811: F, t17815: F, t17818: F, t17822: F, t1818: F, t3666: F, t3670: F, t3755: F, t3756: F, t3767: F, t5443: F, t5452: F, t5463: F, t1284: F, t5216: F, t1204: F, t5477: F, t17814: F, t3302: F, t3588: F, t471: F, t3781: F, t3584: F, t5457: F, t5351: F, t3766: F, t1280: F, t17345: F, t17389: F, t17600: F, t1248: F, t5412: F, t12723: F, t1281: F, t1288: F, t12987: F, t17289: F, t17307: F, t1825: F, t3552: F, t3751: F, t3782: F, t5449: F, t5459: F, t5466: F, t5478: F, t5481: F, t5494: F, t3568: F, t5486: F, t1794: F, t3727: F, t1770: F, t3759: F, t5245: F, t13126: F, t5462: F, t3754: F, t5219: F, t12699: F, t12709: F, t17331: F, t1822: F, t3746: F, t3770: F, t3774: F, t3778: F, t3787: F, t490: F, t5436: F, t5446: F, t5470: F, t5491: F, t17186: F, t1277: F, t1828: F, t3738: F, t13182: F, t3566: F, t488: F, t1276: F, t1774: F, t3575: F, t17807: F, t225: F, t494: F, t1209: F, t3736: F, t3790: F, t3737: F, t1811: F, t1210: F, t12654: F, t1271: F, t1274: F, t1829: F, t3556: F, t3569: F, t3572: F, t3576: F, t3739: F, t5220: F, t5225: F, t5237: F, t5246: F, t1214: F, t5497: F, t1211: F, t3555: F, t1215: F, t12628: F, t12633: F, t12641: F, t12658: F, t1295: F, t13177: F, t1775: F, t3561: F, t3732: F, t495: F, t5231: F, t5251: F, t5417: F, t5423: F, t5429: F, t5498: F, t1294: F, t17306: F) -> (F, F, F, F, F, F, F) {
        let (t17826, t17829, t17834, t17837, t17840, t17845) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1449::<F>(t1269, t1287, t5284, t17633, t5458, t17482, t3769, t3783, t12713, t5332, t13147, t487);
        let t17859 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1450::<F>(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let (t17861, t17864, t17869, t17876, t17879) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1451::<F>(t1284, t5216, t1204, t5477, t17814, t3783, t3302, t3588, t471, t5332, t1269, t3781);
        let (t17880, t17884, t17888, t17893, t17902, t17905) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1452::<F>(t17879, t460, t3584, t5457, t5351, t1269, t3766, t1280, t17345, t1287, t17389, t17600);
        let t17912 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1453::<F>(t1248, t1287, t5412, t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
        let (t17917, t17921, t17934, t17941, t17945, t17948) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1454::<F>(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
        let t17961 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1455::<F>(t17948, t460, t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219, t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17964, t17968, t17973, t17974) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1456::<F>(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488, t1276, t1774);
        let (t17975, t17979, t17986, t17988, t17992, t17995) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1457::<F>(t17974, t3575, t17807, t225, t494, t1209, t488, t1828, t3736, t3790, t3737, t1811, t3566);
        let t18004 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1458::<F>(t1828, t3584, t1277, t1210, t12654, t1271, t1274, t17964, t17968, t17973, t17975, t17979, t17986, t17988, t17992, t17995, t1829, t3556, t3569, t3572, t3576, t3739, t460, t5216, t5220, t5225, t5237, t5246);
        let t18040 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1459::<F>(t1269, t1770, t1214, t5497, t1277, t1211, t17345, t1811, t3555, t1210, t1215, t12628, t12633, t12641, t12658, t1295, t13177, t17331, t1775, t3561, t3572, t3576, t3732, t3739, t495, t5231, t5251, t5417, t5423, t5429, t5498);
        let (t18043, t18047, t18054, t18059, t18062) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1460::<F>(t1294, t5245, t1277, t1774, t3737, t3738, t460, t5412, t17306, t487, t1269, t5219);
    (t18004, t18040, t18043, t18047, t18054, t18059, t18062)
}
