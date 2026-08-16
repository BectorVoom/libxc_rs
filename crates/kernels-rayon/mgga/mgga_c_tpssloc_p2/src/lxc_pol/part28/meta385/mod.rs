//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta385 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1492;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1493;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1494;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1495;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1496;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1497;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1498;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1499;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1500;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1501;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1502;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta385(t15621: f64, t4582: f64, t11721: f64, t3507: f64, t4977: f64, t3509: f64, t1216: f64, t15553: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64, t1227: f64, t11705: f64, t11719: f64, t11728: f64, t11734: f64, t11746: f64, t15610: f64, t15612: f64, t15617: f64, t3490: f64, t3496: f64, t3515: f64, t4974: f64, t4984: f64, t5019: f64, t12652: f64, t4972: f64, t11153: f64, t3584: f64, t14165: f64, t1734: f64, t3508: f64, t1089: f64, t1215: f64, t607: f64, t3578: f64, t1196: f64, t12606: f64, t974: f64, t3548: f64, t4889: f64, t14736: f64, t3440: f64, t14740: f64, t11678: f64, t1174: f64, t11755: f64, t11787: f64, t11792: f64, t11794: f64, t11798: f64, t11802: f64, t11821: f64, t14731: f64, t135: f64, t5045: f64, t1222: f64, t4966: f64, t475: f64, t4728: f64, t1735: f64, t3243: f64, t11668: f64, t1744: f64, t3540: f64, t1731: f64, t4961: f64, t1743: f64, t3566: f64, t11692: f64, t11834: f64, t3552: f64, t3557: f64, t3562: f64, t3577: f64, t488: f64, t1706: f64, t3545: f64, t11818: f64, t248: f64, t1213: f64, t11789: f64, t1653: f64, t15437: f64, t3505: f64, t3576: f64, t5064: f64, t4988: f64, t4723: f64, t1725: f64, t698: f64, t1230: f64, t14706: f64, t15426: f64, t68: f64, t484: f64, t11836: f64, t11839: f64, t11842: f64, t3511: f64, t3580: f64, t3587: f64, t5024: f64, t5030: f64, t15466: f64, t15512: f64, t15558: f64, t15601: f64, t493: f64, t5052: f64, t1246: f64, t11888: f64, t11904: f64, t11907: f64, t11914: f64, t1201: f64, t1244: f64, t1247: f64, t15032: f64, t15241: f64, t15245: f64, t15248: f64, t15253: f64, t15257: f64, t15430: f64, t1758: f64, t3565: f64, t3604: f64, t3610: f64, t3621: f64, t3624: f64, t3626: f64, t470: f64, t494: f64, t5069: f64, t5076: f64, t5080: f64, t5084: f64, t5086: f64) -> (f64, f64) {
        let (t15622, t15627, t15631, t15637, t15642, t15643) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1492(t15621, t4582, t11721, t3507, t4977, t3509, t1216, t15553, t13969, t4979, t3506, t4973);
        let t15648 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1493(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
        let (t15650, t15656, t15663) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1494(t12652, t4972, t4582, t11153, t3584, t14165, t1734, t3508, t1089, t1215, t607, t3578);
        let t15684 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1495(t1196, t12606, t974, t3548, t4889, t14736, t3440, t14740, t11678, t1174, t11755, t11787, t11792, t11794, t11798, t11802, t11821, t1227, t15650, t15656, t15663);
        let (t15686, t15691, t15699, t15700, t15702) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1496(t14731, t3440, t135, t5045, t1174, t1222, t4966, t1215, t1734, t1089, t475, t607);
        let (t15704, t15708, t15710, t15714, t15717) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1497(t15700, t15702, t3578, t1215, t607, t475, t4728, t1735, t3243, t11668, t1744, t3540);
        let t15726 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1498(t1731, t3540, t1222, t4961, t1743, t3566, t11692, t1174, t11834, t15686, t15691, t15699, t15704, t15710, t15714, t15717, t3552, t3557, t3562, t3577, t488, t4889);
        let (t15727, t15731, t15735, t15737, t15740) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1499(t1706, t3545, t11818, t1735, t248, t1213, t11789, t1653, t1227, t15437, t3505, t3576, t5064);
        let (t15745, t15750, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1500(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let t15768 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1501(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15777) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1502(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
        let t15785 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1503(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
    (t15771, t15785)
}
