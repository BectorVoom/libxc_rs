//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta668 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2362;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta668(t12734: f64, t7461: f64, t2314: f64, t25980: f64, t22574: f64, t56120: f64, t8643: f64, t1845: f64, t3719: f64, t1874: f64, t55962: f64, t19456: f64, t6525: f64, t22480: f64, t4028: f64, t12545: f64, t1774: f64, t22461: f64, t22600: f64, t2364: f64, t24999: f64, t25965: f64, t4077: f64, t6517: f64, t7472: f64, t91578: f64, t91580: f64, t91582: f64, t91585: f64, t91587: f64, t91589: f64, t26502: f64, t532: f64, t1983: f64, t6879: f64, t26142: f64, t4034: f64, t1266: f64, t26135: f64, t652: f64, t24987: f64, t6997: f64, t22591: f64, t24990: f64, t6880: f64, t22573: f64, t7684: f64, t22575: f64, t22585: f64, t7685: f64, t12725: f64, t12813: f64, t1976: f64, t22483: f64, t2312: f64, t2323: f64, t24983: f64, t25958: f64, t3652: f64, t4026: f64, t650: f64, t6539: f64, t671: f64, t6862: f64, t7451: f64, t7670: f64, t22607: f64, t7754: f64, t6875: f64, t8944: f64, t26164: f64, t22578: f64, t7753: f64, t7756: f64, t531: f64, t7752: f64, t22596: f64, t16153: f64, t24995: f64, t8945: f64, t25988: f64, t31035: f64, t2018: f64, t40611: f64, t3698: f64, t26161: f64, t15868: f64, t6996: f64, t3734: f64, t23831: f64, t7458: f64, t9348: f64, t12724: f64, t12823: f64, t12835: f64, t24980: f64, t3929: f64, t7681: f64, t22479: f64, t7468: f64, t15904: f64, t33136: f64, t26003: f64, t90381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91591, t91593, t91602, t91606, t91608, t91610) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2355(t12734, t7461, t2314, t25980, t22574, t56120, t8643, t1845, t3719, t1874, t55962, t19456, t6525);
        let t91617 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2356(t22480, t4028, t12545, t12734, t1774, t22461, t22600, t2314, t2364, t24999, t25965, t4077, t6517, t7472, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593, t91602, t91606, t91608, t91610);
        let (t91623, t91625, t91627, t91630, t91637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2357(t26502, t532, t1983, t6879, t2314, t26142, t4034, t1266, t26135, t652, t24987, t6997);
        let t91663 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2358(t1983, t22591, t24990, t24987, t6880, t22573, t7684, t22575, t22585, t7685, t12725, t12813, t1976, t22483, t2312, t2314, t2323, t24983, t24999, t25958, t3652, t4026, t4028, t650, t652, t6539, t671, t6862, t7451, t7670, t91623, t91625, t91627, t91630, t91637);
        let (t91666, t91671, t91673, t91674, t91678) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359(t22607, t7754, t6875, t8944, t26164, t1983, t22578, t7753, t7756, t531, t7752, t22596);
        let (t91681, t91684, t91690, t91694) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2360(t16153, t24995, t8945, t22574, t25988, t31035, t2018, t40611, t1845, t3698, t26161, t15868, t1983, t6996);
        let t91709 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361(t1845, t3734, t24995, t8643, t23831, t7458, t22480, t7461, t9348, t12724, t12823, t12835, t1976, t2314, t24980, t25965, t3929, t4034, t6517, t7472, t7681, t91666, t91671, t91673, t91674, t91678, t91681, t91684, t91690, t91694);
        let (t91713, t91715, t91718, t91722, t91724, t91726) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2362(t1774, t22479, t652, t7468, t9348, t15904, t22574, t33136, t12734, t2314, t26003, t1874, t90381);
    (t91617, t91663, t91709, t91713, t91715, t91718, t91722, t91724, t91726)
}
