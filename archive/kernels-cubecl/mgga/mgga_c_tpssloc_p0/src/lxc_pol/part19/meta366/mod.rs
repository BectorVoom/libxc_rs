//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta366 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1332;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1333;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1336;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1338;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1339;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1340;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1342;
use chunk11::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1343;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta366<F: Float>(t10283: F, t969: F, t10189: F, t3014: F, t2986: F, t2990: F, t10346: F, t2987: F, t10190: F, t10245: F, t10250: F, t13779: F, t10186: F, t10196: F, t10241: F, t10246: F, t10255: F, t10259: F, t10260: F, t346: F, t42759: F, t2989: F, t9258: F, t10337: F, t964: F, t340: F, t625: F, t221: F, t339: F, t344: F, t10256: F, t10328: F, t2960: F, t2988: F, t41644: F, t41649: F, t41705: F, t41715: F, t4510: F, t4518: F, t10195: F, t13784: F, t1887: F, t2262: F, t337: F, t10191: F, t13783: F, t984: F, t10237: F, t10277: F, t343: F, t9288: F, t4509: F, t10273: F, t10231: F, t10279: F, t973: F, t10235: F, t10238: F, t10242: F, t13798: F, t2991: F, t41693: F, t42308: F, t974: F, t41666: F, t10224: F, t2999: F, t2978: F, t698: F, t2981: F, t10263: F, t2971: F, t2402: F, t976: F, t979: F, t2955: F, t2967: F, t10209: F, t10217: F, t10325: F, t2979: F, t3000: F, t39097: F, t42554: F, t4546: F, t980: F, t987: F, t986: F, t3010: F, t10327: F, t135: F, t10286: F, t3016: F, t10289: F, t2974: F, t10348: F, t10349: F, t3011: F, t10352: F, t10232: F, t10208: F, t13822: F, t2995: F, t10228: F, t10280: F, t23547: F, t2980: F, t2982: F, t2994: F, t2996: F, t3008: F, t3017: F, t39103: F, t977: F, t10225: F, t10213: F, t10218: F, t41687: F, t10236: F, t10913: F, t41831: F, t41833: F, t41836: F, t41839: F, t41842: F, t41887: F, t41889: F, t41892: F, t41964: F, t41967: F, t41970: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42762, t42773, t42775, t42785, t42788) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1332::<F>(t10283, t969, t10189, t3014, t2986, t2990, t10346, t2987, t10190, t10245, t10250, t13779);
        let t42790 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1333::<F>(t10186, t10196, t10241, t10246, t10255, t10259, t10260, t2986, t2990, t346, t42759, t42762, t42773, t42775, t42785, t42788);
        let (t42794, t42799, t42811, t42813, t42817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334::<F>(t10190, t10255, t2986, t2989, t9258, t10337, t964, t340, t625, t221, t339, t344);
        let t42824 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335::<F>(t10186, t10241, t10245, t10256, t10328, t2960, t2986, t2988, t41644, t41649, t41705, t41715, t42794, t42799, t42811, t42817, t4510, t4518);
        let (t42827, t42830, t42833, t42839, t42841) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1336::<F>(t10195, t13784, t2986, t1887, t2262, t337, t10186, t10191, t13783, t984, t10237, t10277, t343);
        let t42860 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337::<F>(t42841, t9288, t3014, t4509, t10273, t2960, t10231, t10279, t973, t10186, t10235, t10237, t10238, t10242, t13798, t2986, t2991, t41693, t42827, t42830, t42833, t42839);
        let (t42861, t42862, t42873, t42877, t42889) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1338::<F>(t42308, t974, t344, t41666, t10224, t2999, t973, t2978, t698, t2981, t10263, t2971);
        let t42899 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1339::<F>(t2402, t976, t973, t979, t2955, t2967, t10209, t10217, t10263, t10325, t2960, t2979, t3000, t343, t39097, t42554, t42861, t42862, t42873, t42877, t42889, t4546, t980, t984, t987);
        let (t42903, t42906, t42909, t42911, t42914, t42916, t42918) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1340::<F>(t2402, t973, t986, t3010, t698, t10327, t135, t10286, t2960, t3016, t10289, t10263, t2974);
        let t42933 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341::<F>(t10348, t135, t973, t3014, t10263, t10349, t2960, t3011, t340, t343, t42903, t42906, t42909, t42911, t42914, t42916, t42918, t974);
        let t42966 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1342::<F>(t10352, t2960, t10232, t10208, t13822, t973, t10224, t2995, t10228, t10263, t10280, t23547, t2979, t2980, t2982, t2994, t2996, t3008, t3017, t39103, t4546, t977);
        let (t42968, t42974, t42976, t42985, t43000) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1343::<F>(t10225, t2960, t10213, t135, t10218, t973, t344, t41687, t10236, t10913, t41831, t41833, t41836, t41839, t41842, t41887, t41889, t41892, t41964, t41967, t41970);
    (t42790, t42813, t42824, t42860, t42899, t42933, t42966, t42968, t42974, t42976, t42985, t43000)
}
