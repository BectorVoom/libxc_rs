//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta367 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1372;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1373;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1374;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1375;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1376;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1377;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1378;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1379;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1380;
use chunk9::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1381;
use chunk10::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1382;
use chunk11::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1383;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta367<F: Float>(t13823: F, t973: F, t2970: F, t4522: F, t6733: F, t884: F, t4531: F, t10254: F, t3961: F, t2988: F, t10236: F, t10235: F, t10186: F, t10233: F, t10267: F, t10274: F, t13806: F, t13813: F, t13817: F, t2960: F, t2986: F, t4523: F, t4532: F, t4549: F, t10189: F, t1597: F, t2990: F, t2987: F, t4540: F, t10245: F, t10241: F, t4514: F, t2989: F, t3966: F, t13542: F, t4518: F, t13546: F, t10259: F, t13559: F, t13555: F, t4510: F, t3014: F, t343: F, t4546: F, t3008: F, t4506: F, t10263: F, t1593: F, t10224: F, t1592: F, t4528: F, t1599: F, t698: F, t135: F, t4542: F, t13552: F, t13550: F, t13644: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13557: F, t13561: F, t13642: F, t13647: F, t340: F, t974: F, t10287: F, t10290: F, t10331: F, t10333: F, t10339: F, t10342: F, t10353: F, t1600: F, t4543: F, t13804: F, t225: F, t68: F, t369: F, t1036: F, t4622: F, t3117: F, t4571: F, t248: F, t3051: F, t4347: F, t1041: F, t10370: F, t10372: F, t10377: F, t10381: F, t10385: F, t10390: F, t13750: F, t13751: F, t13758: F, t13762: F, t13767: F, t3070: F, t378: F, t4579: F, t3114: F, t4630: F, t3101: F, t4650: F, t1020: F, t10508: F, t1616: F, t122: F, t247: F, t4599: F, t3039: F, t376: F, t4649: F, t4594: F, t4582: F, t3120: F, t3131: F, t4593: F, t10482: F, t3040: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13825, t13830, t13832, t13836, t13840) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1372::<F>(t13823, t973, t2970, t4522, t6733, t884, t4531, t10254, t3961, t2988, t10236, t10235);
        let t13845 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1373::<F>(t10186, t10233, t10267, t10274, t13806, t13813, t13817, t13825, t13830, t13832, t13836, t13840, t2960, t2986, t4523, t4532, t4549, t973);
        let (t13850, t13852, t13855, t13858, t13861) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1374::<F>(t10189, t1597, t2990, t2986, t2987, t4540, t10245, t4531, t10241, t4514, t2989, t3966);
        let (t13862, t13865, t13868, t13871, t13874, t13877, t13881) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1375::<F>(t13861, t2988, t13542, t4518, t13546, t10259, t4514, t13559, t13555, t4510, t1597, t3014, t343);
        let t13894 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1376::<F>(t13881, t4546, t1597, t3008, t343, t2960, t4506, t10263, t13850, t13852, t13855, t13858, t13862, t13865, t13868, t13871, t13874, t13877, t1593, t2986, t973);
        let (t13896, t13907, t13909, t13915, t13921, t13922, t13923) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1377::<F>(t10224, t1592, t973, t2960, t4528, t1599, t698, t135, t4542, t13552, t13550, t13644);
        let t13931 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1378::<F>(t10295, t10296, t10298, t10300, t10302, t13530, t13534, t13539, t13544, t13548, t13557, t13561, t13642, t13647, t13921, t13922, t13923);
        let t13937 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1379::<F>(t13931, t340, t343, t974, t10263, t10287, t10290, t10331, t10333, t10339, t10342, t10353, t13896, t13907, t13909, t13915, t1600, t2960, t4543, t973);
        let (t13939, t13940, t13942, t13946, t13948, t13950) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1380::<F>(t13804, t13845, t13894, t13937, t225, t68, t369, t1036, t4622, t3117, t4571, t248, t3051, t4347);
        let t13953 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1381::<F>(t1041, t13950, t10370, t10372, t10377, t10381, t10385, t10390, t13750, t13751, t13758, t13762, t13767, t13942, t13946, t13948, t3070, t378, t4579);
        let (t13959, t13963, t13966, t13969) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1382::<F>(t3114, t4630, t248, t3101, t4650, t1020, t10508, t1616, t122, t247);
        let (t13972, t13975, t13977, t13982, t13985) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1383::<F>(t13969, t4599, t3039, t376, t4649, t4594, t4582, t3120, t3131, t4593, t10482, t3040);
    (t13939, t13940, t13953, t13959, t13963, t13966, t13969, t13972, t13975, t13977, t13982, t13985)
}
