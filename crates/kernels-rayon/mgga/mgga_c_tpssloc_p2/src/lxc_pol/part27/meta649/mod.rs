//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta649 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2251;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2252;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2254;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2255;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2256;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2257;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2258;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2259;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2260;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta649(t25836: f64, t3216: f64, t11094: f64, t7627: f64, t1068: f64, t1070: f64, t14662: f64, t1637: f64, t193: f64, t23738: f64, t23742: f64, t25840: f64, t25845: f64, t3209: f64, t3213: f64, t336: f64, t4696: f64, t4700: f64, t60941: f64, t6822: f64, t83468: f64, t83472: f64, t83479: f64, t88054: f64, t88097: f64, t88137: f64, t88179: f64, t88213: f64, t88742: f64, t88779: f64, t88827: f64, t88867: f64, t88900: f64, t88940: f64, t89556: f64, t89590: f64, t89623: f64, t89658: f64, t89690: f64, t12915: f64, t13487: f64, t13191: f64, t13471: f64, t1530: f64, t16596: f64, t1877: f64, t1915: f64, t202: f64, t22959: f64, t23290: f64, t2379: f64, t25013: f64, t2522: f64, t25358: f64, t25365: f64, t25374: f64, t2553: f64, t4119: f64, t4314: f64, t57893: f64, t57912: f64, t6666: f64, t6670: f64, t7541: f64, t81525: f64, t81539: f64, t82312: f64, t86717: f64, t868: f64, t86836: f64, t870: f64, t87944: f64, t12971: f64, t13196: f64, t1484: f64, t23286: f64, t23295: f64, t25354: f64, t2745: f64, t2749: f64, t4255: f64, t4303: f64, t47645: f64, t57921: f64, t58009: f64, t58071: f64, t59580: f64, t7634: f64, t776: f64, t86706: f64, t86713: f64, t86815: f64, t87975: f64, t25: f64, t265: f64, t394: f64, t12606: f64, t1409: f64, t1965: f64, t2250: f64, t23773: f64, t25883: f64, t3966: f64, t40: f64, t607: f64, t6835: f64, t7643: f64, t88003: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t23788: f64, t25891: f64, t25927: f64, t10143: f64, t1081: f64, t1649: f64, t23789: f64, t23792: f64, t25372: f64, t6848: f64, t86736: f64, t28: f64, t3231: f64, t86781: f64, t25928: f64, t25945: f64, t7649: f64, t86703: f64, t86734: f64, t86751: f64, t86757: f64, t87945: f64, t86797: f64, t83555: f64, t40772: f64, t23781: f64, t23807: f64, t23810: f64, t23813: f64, t25892: f64, t25898: f64, t25905: f64, t6841: f64, t81483: f64, t86740: f64, t86775: f64, t86835: f64, t2752: f64, t23796: f64, t25901: f64, t25921: f64, t25930: f64, t25934: f64, t25938: f64, t7650: f64, t7656: f64, t504: f64, t1972: f64, t23821: f64, t25950: f64, t52: f64, t6856: f64, t7664: f64, rho1: f64, t23858: f64, t7685: f64, t22607: f64, t7688: f64, t1390: f64, t16018: f64, t1983: f64, t6878: f64, t22574: f64, t56194: f64, t8643: f64, t113: f64, t1393: f64, t1459: f64, t26138: f64, t4072: f64, t5107: f64, t6515: f64, t652: f64, t6862: f64, t83935: f64, t86673: f64, t86676: f64, t86679: f64, t86682: f64, t86684: f64, t86688: f64, t86693: f64, t86698: f64, t86700: f64, t86702: f64) -> f64 {
        let t89729 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2250(t25836, t3216, t11094, t7627, t1068, t1070, t14662, t1637, t193, t23738, t23742, t25840, t25845, t3209, t3213, t336, t4696, t4700, t60941, t6822, t83468, t83472, t83479, t88054, t88097, t88137, t88179, t88213, t88742, t88779, t88827, t88867, t88900, t88940, t89556, t89590, t89623, t89658, t89690);
        let t89775 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2251(t12915, t13487, t13191, t13471, t1530, t16596, t1877, t1915, t193, t202, t22959, t23290, t2379, t25013, t2522, t25358, t25365, t25374, t2553, t4119, t4314, t57893, t57912, t6666, t6670, t7541, t81525, t81539, t82312, t86717, t868, t86836, t870, t87944);
        let t89822 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2252(t12971, t13196, t1484, t1877, t1915, t23286, t23290, t23295, t2522, t25354, t25358, t2745, t2749, t4255, t4303, t4314, t47645, t57921, t58009, t58071, t59580, t6666, t6670, t7634, t776, t86706, t86713, t86815, t87975);
        let (t89823, t89836) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2253(t25, t265, t394, t89775, t89822, t89729, t12606, t1409, t1965, t2250, t23773, t25883, t3966, t40, t607, t6835, t7643, t88003, dens_threshold, rho0, zeta_threshold);
        let (t89837, t89840, t89843, t89846, t89850, t89859) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2254(t23788, t59580, t86815, t13196, t25891, t25927, t58009, t10143, t1081, t25374, t4255, t870);
        let t89880 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2255(t23788, t58071, t86706, t1649, t2745, t25927, t86713, t2379, t1877, t1915, t22959, t23789, t23792, t25013, t2522, t25372, t4314, t6670, t6848, t7541, t86736, t86836, t89837, t89840, t89843, t89846, t89850, t89859);
        let (t89881, t89888, t89892, t89896, t89904, t89907, t89911) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2256(t1649, t2553, t12971, t28, t1081, t4119, t13191, t25891, t25927, t57921, t13471, t1484, t3231);
        let t89920 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2257(t25927, t86781, t1877, t1915, t22959, t23286, t23290, t25013, t2522, t25928, t25945, t28, t6670, t7649, t86703, t86734, t86751, t86757, t87945, t89881, t89888, t89892, t89896, t89904, t89907, t89911);
        let t89957 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2258(t23788, t86797, t16596, t83555, t1081, t4303, t28, t40772, t86717, t1877, t22959, t23781, t23807, t23810, t23813, t25013, t2522, t25354, t25358, t25372, t25892, t25898, t25905, t4314, t6666, t6670, t6841, t7541, t81483, t86740, t86775, t86835, t87975);
        let t90001 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2259(t25365, t83555, t1530, t3231, t1649, t2749, t23788, t57893, t2752, t13487, t1877, t22959, t23286, t23290, t23295, t23796, t2522, t25901, t25921, t25930, t25934, t25938, t47645, t6666, t6670, t7541, t7650, t7656, t81483, t81525);
        let t90016 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2260(t28, t265, t504, t89880, t89920, t89957, t90001, t89823, t12606, t1409, t1972, t2250, t23821, t25950, t3966, t52, t607, t6856, t7664, dens_threshold, rho1, zeta_threshold);
        let t90030 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2261(t23858, t7685, t22607, t7688, t1390, t16018, t1983, t6878, t22574, t56194, t8643, t113, t1393, t1459, t26138, t4072, t5107, t6515, t652, t6862, t83935, t86673, t86676, t86679, t86682, t86684, t86688, t86693, t86698, t86700, t86702, t89836, t90016);
    t90030
}
