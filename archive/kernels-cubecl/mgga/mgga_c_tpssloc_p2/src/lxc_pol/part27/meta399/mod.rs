//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta399 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1652;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1653;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1654;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1655;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1656;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1657;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1658;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1659;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1660;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1661;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta399<F: Float>(t15621: F, t4582: F, t11721: F, t3507: F, t4977: F, t3509: F, t1216: F, t15553: F, t13969: F, t4979: F, t3506: F, t4973: F, t1227: F, t11705: F, t11719: F, t11728: F, t11734: F, t11746: F, t15610: F, t15612: F, t15617: F, t3490: F, t3496: F, t3515: F, t4974: F, t4984: F, t5019: F, t12652: F, t4972: F, t11153: F, t3584: F, t14165: F, t1734: F, t3508: F, t1089: F, t1215: F, t607: F, t3578: F, t1196: F, t12606: F, t974: F, t3548: F, t4889: F, t14736: F, t3440: F, t14740: F, t11678: F, t1174: F, t11755: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11821: F, t14731: F, t135: F, t5045: F, t1222: F, t4966: F, t475: F, t4728: F, t1735: F, t3243: F, t11668: F, t1744: F, t3540: F, t1731: F, t4961: F, t1743: F, t3566: F, t11692: F, t11834: F, t3552: F, t3557: F, t3562: F, t3577: F, t488: F, t1706: F, t3545: F, t11818: F, t248: F, t1213: F, t11789: F, t1653: F, t15437: F, t3505: F, t3576: F, t5064: F, t4988: F, t4723: F, t1725: F, t698: F, t1230: F, t14706: F, t15426: F, t68: F, t484: F, t11836: F, t11839: F, t11842: F, t3511: F, t3580: F, t3587: F, t5024: F, t5030: F, t15466: F, t15512: F, t15558: F, t15601: F, t493: F, t5052: F, t1246: F, t11888: F, t11904: F, t11907: F, t11914: F, t1201: F, t1244: F, t1247: F, t15032: F, t15241: F, t15245: F, t15248: F, t15253: F, t15257: F, t15430: F, t1758: F, t3565: F, t3604: F, t3610: F, t3621: F, t3624: F, t3626: F, t470: F, t494: F, t5069: F, t5076: F, t5080: F, t5084: F, t5086: F) -> (F, F) {
        let (t15622, t15627, t15631, t15637, t15642, t15643) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1651::<F>(t15621, t4582, t11721, t3507, t4977, t3509, t1216, t15553, t13969, t4979, t3506, t4973);
        let t15648 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1652::<F>(t1227, t15643, t11705, t11719, t11728, t11734, t11746, t15610, t15612, t15617, t15622, t15627, t15631, t15637, t15642, t3490, t3496, t3506, t3515, t4974, t4984, t5019);
        let (t15650, t15656, t15663) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1653::<F>(t12652, t4972, t4582, t11153, t3584, t14165, t1734, t3508, t1089, t1215, t607, t3578);
        let t15684 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1654::<F>(t1196, t12606, t974, t3548, t4889, t14736, t3440, t14740, t11678, t1174, t11755, t11787, t11792, t11794, t11798, t11802, t11821, t1227, t15650, t15656, t15663);
        let (t15686, t15691, t15699, t15700, t15702) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1655::<F>(t14731, t3440, t135, t5045, t1174, t1222, t4966, t1215, t1734, t1089, t475, t607);
        let (t15704, t15708, t15710, t15714, t15717) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1656::<F>(t15700, t15702, t3578, t1215, t607, t475, t4728, t1735, t3243, t11668, t1744, t3540);
        let t15726 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1657::<F>(t1731, t3540, t1222, t4961, t1743, t3566, t11692, t1174, t11834, t15686, t15691, t15699, t15704, t15710, t15714, t15717, t3552, t3557, t3562, t3577, t488, t4889);
        let (t15727, t15731, t15735, t15737, t15740) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1658::<F>(t1706, t3545, t11818, t1735, t248, t1213, t11789, t1653, t1227, t15437, t3505, t3576, t5064);
        let (t15745, t15750, t15754, t15761) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1659::<F>(t13969, t4988, t1227, t15708, t4723, t11668, t1725, t698, t1174, t1230, t14706, t248);
        let t15768 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1660::<F>(t15426, t68, t484, t11836, t11839, t11842, t1227, t15727, t15731, t15735, t15737, t15740, t15745, t15750, t15754, t15761, t3490, t3511, t3577, t3580, t3587, t488, t5024, t5030);
        let (t15771, t15772, t15777) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1661::<F>(t15466, t15512, t15558, t15601, t15648, t15684, t15726, t15768, t493, t1215, t5052, t1246);
        let t15785 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1662::<F>(t11888, t11904, t11907, t11914, t1201, t1244, t1247, t15032, t15241, t15245, t15248, t15253, t15257, t15426, t15430, t15772, t15777, t1758, t3565, t3604, t3610, t3621, t3624, t3626, t470, t494, t5064, t5069, t5076, t5080, t5084, t5086);
    (t15771, t15785)
}
