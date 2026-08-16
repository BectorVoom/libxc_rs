//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta379 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1515;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1516;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1517;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1518;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1519;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1520;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1521;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1522;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1523;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1524;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1525;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1526;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta379(t13823: f64, t973: f64, t2970: f64, t4522: f64, t6733: f64, t884: f64, t4531: f64, t10254: f64, t3961: f64, t2988: f64, t10236: f64, t10235: f64, t10186: f64, t10233: f64, t10267: f64, t10274: f64, t13806: f64, t13813: f64, t13817: f64, t2960: f64, t2986: f64, t4523: f64, t4532: f64, t4549: f64, t10189: f64, t1597: f64, t2990: f64, t2987: f64, t4540: f64, t10245: f64, t10241: f64, t4514: f64, t2989: f64, t3966: f64, t13542: f64, t4518: f64, t13546: f64, t10259: f64, t13559: f64, t13555: f64, t4510: f64, t3014: f64, t343: f64, t4546: f64, t3008: f64, t4506: f64, t10263: f64, t1593: f64, t10224: f64, t1592: f64, t4528: f64, t1599: f64, t698: f64, t135: f64, t4542: f64, t13552: f64, t13550: f64, t13644: f64, t10295: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13557: f64, t13561: f64, t13642: f64, t13647: f64, t340: f64, t974: f64, t10287: f64, t10290: f64, t10331: f64, t10333: f64, t10339: f64, t10342: f64, t10353: f64, t1600: f64, t4543: f64, t13804: f64, t225: f64, t68: f64, t369: f64, t1036: f64, t4622: f64, t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64, t1041: f64, t10370: f64, t10372: f64, t10377: f64, t10381: f64, t10385: f64, t10390: f64, t13750: f64, t13751: f64, t13758: f64, t13762: f64, t13767: f64, t3070: f64, t378: f64, t4579: f64, t3114: f64, t4630: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64, t122: f64, t247: f64, t4599: f64, t3039: f64, t376: f64, t4649: f64, t4594: f64, t4582: f64, t3120: f64, t3131: f64, t4593: f64, t10482: f64, t3040: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13825, t13830, t13832, t13836, t13840) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1515(t13823, t973, t2970, t4522, t6733, t884, t4531, t10254, t3961, t2988, t10236, t10235);
        let t13845 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1516(t10186, t10233, t10267, t10274, t13806, t13813, t13817, t13825, t13830, t13832, t13836, t13840, t2960, t2986, t4523, t4532, t4549, t973);
        let (t13850, t13852, t13855, t13858, t13861) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1517(t10189, t1597, t2990, t2986, t2987, t4540, t10245, t4531, t10241, t4514, t2989, t3966);
        let (t13862, t13865, t13868, t13871, t13874, t13877, t13881) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1518(t13861, t2988, t13542, t4518, t13546, t10259, t4514, t13559, t13555, t4510, t1597, t3014, t343);
        let t13894 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1519(t13881, t4546, t1597, t3008, t343, t2960, t4506, t10263, t13850, t13852, t13855, t13858, t13862, t13865, t13868, t13871, t13874, t13877, t1593, t2986, t973);
        let (t13896, t13907, t13909, t13915, t13921, t13922, t13923) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1520(t10224, t1592, t973, t2960, t4528, t1599, t698, t135, t4542, t13552, t13550, t13644);
        let t13931 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1521(t10295, t10296, t10298, t10300, t10302, t13530, t13534, t13539, t13544, t13548, t13557, t13561, t13642, t13647, t13921, t13922, t13923);
        let t13937 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1522(t13931, t340, t343, t974, t10263, t10287, t10290, t10331, t10333, t10339, t10342, t10353, t13896, t13907, t13909, t13915, t1600, t2960, t4543, t973);
        let (t13939, t13940, t13942, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1523(t13804, t13845, t13894, t13937, t225, t68, t369, t1036, t4622, t3117, t4571, t248, t3051, t4347);
        let t13953 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1524(t1041, t13950, t10370, t10372, t10377, t10381, t10385, t10390, t13750, t13751, t13758, t13762, t13767, t13942, t13946, t13948, t3070, t378, t4579);
        let (t13959, t13963, t13966, t13969) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1525(t3114, t4630, t248, t3101, t4650, t1020, t10508, t1616, t122, t247);
        let (t13972, t13975, t13977, t13982, t13985) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1526(t13969, t4599, t3039, t376, t4649, t4594, t4582, t3120, t3131, t4593, t10482, t3040);
    (t13939, t13940, t13953, t13959, t13963, t13966, t13969, t13972, t13975, t13977, t13982, t13985)
}
