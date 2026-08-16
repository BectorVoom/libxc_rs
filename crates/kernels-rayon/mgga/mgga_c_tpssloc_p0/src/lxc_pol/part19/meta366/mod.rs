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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta366(t10283: f64, t969: f64, t10189: f64, t3014: f64, t2986: f64, t2990: f64, t10346: f64, t2987: f64, t10190: f64, t10245: f64, t10250: f64, t13779: f64, t10186: f64, t10196: f64, t10241: f64, t10246: f64, t10255: f64, t10259: f64, t10260: f64, t346: f64, t42759: f64, t2989: f64, t9258: f64, t10337: f64, t964: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64, t10256: f64, t10328: f64, t2960: f64, t2988: f64, t41644: f64, t41649: f64, t41705: f64, t41715: f64, t4510: f64, t4518: f64, t10195: f64, t13784: f64, t1887: f64, t2262: f64, t337: f64, t10191: f64, t13783: f64, t984: f64, t10237: f64, t10277: f64, t343: f64, t9288: f64, t4509: f64, t10273: f64, t10231: f64, t10279: f64, t973: f64, t10235: f64, t10238: f64, t10242: f64, t13798: f64, t2991: f64, t41693: f64, t42308: f64, t974: f64, t41666: f64, t10224: f64, t2999: f64, t2978: f64, t698: f64, t2981: f64, t10263: f64, t2971: f64, t2402: f64, t976: f64, t979: f64, t2955: f64, t2967: f64, t10209: f64, t10217: f64, t10325: f64, t2979: f64, t3000: f64, t39097: f64, t42554: f64, t4546: f64, t980: f64, t987: f64, t986: f64, t3010: f64, t10327: f64, t135: f64, t10286: f64, t3016: f64, t10289: f64, t2974: f64, t10348: f64, t10349: f64, t3011: f64, t10352: f64, t10232: f64, t10208: f64, t13822: f64, t2995: f64, t10228: f64, t10280: f64, t23547: f64, t2980: f64, t2982: f64, t2994: f64, t2996: f64, t3008: f64, t3017: f64, t39103: f64, t977: f64, t10225: f64, t10213: f64, t10218: f64, t41687: f64, t10236: f64, t10913: f64, t41831: f64, t41833: f64, t41836: f64, t41839: f64, t41842: f64, t41887: f64, t41889: f64, t41892: f64, t41964: f64, t41967: f64, t41970: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42762, t42773, t42775, t42785, t42788) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1332(t10283, t969, t10189, t3014, t2986, t2990, t10346, t2987, t10190, t10245, t10250, t13779);
        let t42790 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1333(t10186, t10196, t10241, t10246, t10255, t10259, t10260, t2986, t2990, t346, t42759, t42762, t42773, t42775, t42785, t42788);
        let (t42794, t42799, t42811, t42813, t42817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334(t10190, t10255, t2986, t2989, t9258, t10337, t964, t340, t625, t221, t339, t344);
        let t42824 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335(t10186, t10241, t10245, t10256, t10328, t2960, t2986, t2988, t41644, t41649, t41705, t41715, t42794, t42799, t42811, t42817, t4510, t4518);
        let (t42827, t42830, t42833, t42839, t42841) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1336(t10195, t13784, t2986, t1887, t2262, t337, t10186, t10191, t13783, t984, t10237, t10277, t343);
        let t42860 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337(t42841, t9288, t3014, t4509, t10273, t2960, t10231, t10279, t973, t10186, t10235, t10237, t10238, t10242, t13798, t2986, t2991, t41693, t42827, t42830, t42833, t42839);
        let (t42861, t42862, t42873, t42877, t42889) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1338(t42308, t974, t344, t41666, t10224, t2999, t973, t2978, t698, t2981, t10263, t2971);
        let t42899 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1339(t2402, t976, t973, t979, t2955, t2967, t10209, t10217, t10263, t10325, t2960, t2979, t3000, t343, t39097, t42554, t42861, t42862, t42873, t42877, t42889, t4546, t980, t984, t987);
        let (t42903, t42906, t42909, t42911, t42914, t42916, t42918) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1340(t2402, t973, t986, t3010, t698, t10327, t135, t10286, t2960, t3016, t10289, t10263, t2974);
        let t42933 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341(t10348, t135, t973, t3014, t10263, t10349, t2960, t3011, t340, t343, t42903, t42906, t42909, t42911, t42914, t42916, t42918, t974);
        let t42966 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1342(t10352, t2960, t10232, t10208, t13822, t973, t10224, t2995, t10228, t10263, t10280, t23547, t2979, t2980, t2982, t2994, t2996, t3008, t3017, t39103, t4546, t977);
        let (t42968, t42974, t42976, t42985, t43000) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1343(t10225, t2960, t10213, t135, t10218, t973, t344, t41687, t10236, t10913, t41831, t41833, t41836, t41839, t41842, t41887, t41889, t41892, t41964, t41967, t41970);
    (t42790, t42813, t42824, t42860, t42899, t42933, t42966, t42968, t42974, t42976, t42985, t43000)
}
