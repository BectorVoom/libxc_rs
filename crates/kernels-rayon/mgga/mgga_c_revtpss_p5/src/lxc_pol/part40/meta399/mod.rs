//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta399 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1452;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1453;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1454;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1455;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1456;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1457;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1458;
use chunk7::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1459;
use chunk8::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1460;
use chunk9::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1461;
use chunk10::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1462;
use chunk11::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta399(t1269: f64, t1287: f64, t5284: f64, t17633: f64, t5458: f64, t17482: f64, t3769: f64, t3783: f64, t12713: f64, t5332: f64, t13147: f64, t487: f64, t460: f64, t12050: f64, t13045: f64, t3601: f64, t17710: f64, t13141: f64, t3603: f64, t1234: f64, t12717: f64, t12751: f64, t12756: f64, t1285: f64, t12966: f64, t12975: f64, t17188: f64, t17192: f64, t17808: f64, t17811: f64, t17815: f64, t17818: f64, t17822: f64, t1818: f64, t3666: f64, t3670: f64, t3755: f64, t3756: f64, t3767: f64, t5443: f64, t5452: f64, t5463: f64, t1284: f64, t5216: f64, t1204: f64, t5477: f64, t17814: f64, t3302: f64, t3588: f64, t471: f64, t3781: f64, t3584: f64, t5457: f64, t5351: f64, t3766: f64, t1280: f64, t17345: f64, t17389: f64, t17600: f64, t1248: f64, t5412: f64, t12723: f64, t1281: f64, t1288: f64, t12987: f64, t17289: f64, t17307: f64, t1825: f64, t3552: f64, t3751: f64, t3782: f64, t5449: f64, t5459: f64, t5466: f64, t5478: f64, t5481: f64, t5494: f64, t3568: f64, t5486: f64, t1794: f64, t3727: f64, t1770: f64, t3759: f64, t5245: f64, t13126: f64, t5462: f64, t3754: f64, t5219: f64, t12699: f64, t12709: f64, t17331: f64, t1822: f64, t3746: f64, t3770: f64, t3774: f64, t3778: f64, t3787: f64, t490: f64, t5436: f64, t5446: f64, t5470: f64, t5491: f64, t17186: f64, t1277: f64, t1828: f64, t3738: f64, t13182: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64, t3575: f64, t17807: f64, t225: f64, t494: f64, t1209: f64, t3736: f64, t3790: f64, t3737: f64, t1811: f64, t1210: f64, t12654: f64, t1271: f64, t1274: f64, t1829: f64, t3556: f64, t3569: f64, t3572: f64, t3576: f64, t3739: f64, t5220: f64, t5225: f64, t5237: f64, t5246: f64, t1214: f64, t5497: f64, t1211: f64, t3555: f64, t1215: f64, t12628: f64, t12633: f64, t12641: f64, t12658: f64, t1295: f64, t13177: f64, t1775: f64, t3561: f64, t3732: f64, t495: f64, t5231: f64, t5251: f64, t5417: f64, t5423: f64, t5429: f64, t5498: f64, t1294: f64, t17306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17826, t17829, t17834, t17837, t17840, t17845) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1452(t1269, t1287, t5284, t17633, t5458, t17482, t3769, t3783, t12713, t5332, t13147, t487);
        let t17859 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1453(t17845, t460, t12050, t13045, t3601, t17710, t13141, t487, t3603, t1234, t12717, t12751, t12756, t1285, t12966, t12975, t17188, t17192, t17808, t17811, t17815, t17818, t17822, t17826, t17829, t17834, t17837, t17840, t1818, t3666, t3670, t3755, t3756, t3767, t5443, t5452, t5463);
        let (t17861, t17864, t17869, t17876, t17879) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1454(t1284, t5216, t1204, t5477, t17814, t3783, t3302, t3588, t471, t5332, t1269, t3781);
        let (t17880, t17884, t17888, t17893, t17902, t17905) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1455(t17879, t460, t3584, t5457, t5351, t1269, t3766, t1280, t17345, t1287, t17389, t17600);
        let t17912 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1456(t1248, t1287, t5412, t1204, t12723, t1281, t1285, t1288, t12987, t17289, t17307, t17861, t17864, t17869, t17876, t17880, t17884, t17888, t17893, t17902, t17905, t1825, t3552, t3666, t3751, t3755, t3782, t5449, t5459, t5466, t5478, t5481, t5494);
        let (t17917, t17921, t17934, t17941, t17945, t17948) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1457(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
        let t17961 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1458(t17948, t460, t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219, t1234, t12699, t12709, t12717, t12723, t1285, t17331, t1770, t17917, t17921, t17934, t17941, t17945, t1822, t3670, t3746, t3756, t3770, t3774, t3778, t3787, t490, t5436, t5446, t5466, t5470, t5491);
        let (t17964, t17968, t17973, t17974) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1459(t17186, t17859, t17912, t17961, t1277, t1828, t3738, t13182, t3566, t488, t1276, t1774);
        let (t17975, t17979, t17986, t17988, t17992, t17995) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1460(t17974, t3575, t17807, t225, t494, t1209, t488, t1828, t3736, t3790, t3737, t1811, t3566);
        let t18004 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1461(t1828, t3584, t1277, t1210, t12654, t1271, t1274, t17964, t17968, t17973, t17975, t17979, t17986, t17988, t17992, t17995, t1829, t3556, t3569, t3572, t3576, t3739, t460, t5216, t5220, t5225, t5237, t5246);
        let t18040 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1462(t1269, t1770, t1214, t5497, t1277, t1211, t17345, t1811, t3555, t1210, t1215, t12628, t12633, t12641, t12658, t1295, t13177, t17331, t1775, t3561, t3572, t3576, t3732, t3739, t495, t5231, t5251, t5417, t5423, t5429, t5498);
        let (t18043, t18047, t18054, t18059, t18062) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1463(t1294, t5245, t1277, t1774, t3737, t3738, t460, t5412, t17306, t487, t1269, t5219);
    (t18004, t18040, t18043, t18047, t18054, t18059, t18062)
}
