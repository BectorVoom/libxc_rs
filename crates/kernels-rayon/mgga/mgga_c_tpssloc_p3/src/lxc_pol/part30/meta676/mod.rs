//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta676 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2113;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta676(t12571: f64, t1410: f64, t26012: f64, t7441: f64, t27971: f64, t645: f64, t72: f64, t1437: f64, t7445: f64, t1863: f64, t27975: f64, t1864: f64, t5445: f64, t2240: f64, t5399: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t26013: f64, t26016: f64, t90114: f64, t90192: f64, t90248: f64, t90251: f64, t90330: f64, t3953: f64, t3961: f64, t3967: f64, t4017: f64, t1433: f64, t4021: f64, t1865: f64, t22523: f64, t22554: f64, t26063: f64, t26067: f64, t26084: f64, t27966: f64, t27972: f64, t6490: f64, t6506: f64, t6510: f64, t7432: f64, t90308: f64, t90312: f64, t641: f64, t19445: f64, t79: f64, t27948: f64, t33: f64, t55921: f64, t6489: f64, t19299: f64, t608: f64, t26083: f64, t26051: f64, t26055: f64, t26090: f64, t27976: f64, t6492: f64, t7442: f64, t7446: f64, t1862: f64, t5392: f64, t1409: f64, t605: f64, t3966: f64, t2235: f64, t17635: f64, t19334: f64, t26045: f64, t26048: f64, t26070: f64, t26073: f64, t26076: f64, t27982: f64, t7435: f64, t26021: f64, t26025: f64, t26028: f64, t27979: f64, t7428: f64, t90182: f64, t90185: f64, t16558: f64, t17686: f64, t17691: f64, t1860: f64, t22502: f64, t22505: f64, t26024: f64, t26044: f64, t27949: f64, t27950: f64, t27953: f64, t27957: f64, t5398: f64, t6486: f64, t6500: f64, t6509: f64, t67: f64, t83791: f64, t83796: f64, t83803: f64, t5: f64, t96409: f64, t96441: f64, t112: f64, t5456: f64, t6514: f64, t19534: f64, t88: f64, t1873: f64, t28007: f64, t6534: f64, t26114: f64, t7467: f64, t26117: f64, t26135: f64, t7676: f64, t2314: f64, t28017: f64, t5113: f64, t96356: f64, t28002: f64, t12725: f64, t75560: f64, t19451: f64) -> (f64, f64, f64) {
        let (t96443, t96454, t96458, t96462, t96466, t96469) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107(t12571, t1410, t26012, t7441, t27971, t645, t72, t1437, t7445, t1863, t27975, t1864, t5445);
        let t96478 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108(t1863, t96469, t2240, t5399, t22544, t22549, t22551, t26009, t26013, t26016, t90114, t90192, t90248, t90251, t90330, t96443, t96454, t96458, t96462, t96466);
        let t96509 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109(t3953, t3961, t3967, t1437, t4017, t72, t1433, t4021, t1865, t22523, t22554, t26063, t26067, t26084, t27966, t27972, t6490, t6506, t6510, t7432, t90308, t90312);
        let (t96517, t96521, t96529, t96532, t96535) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110(t5445, t641, t72, t19445, t79, t2240, t27948, t33, t55921, t6489, t19299, t608);
        let t96545 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111(t12571, t26083, t1865, t22523, t22554, t26051, t26055, t26067, t26090, t27976, t6490, t6492, t7442, t7446, t96517, t96521, t96529, t96532, t96535);
        let (t96547, t96551, t96553, t96556, t96559, t96562) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112(t1862, t2240, t5392, t1409, t605, t3966, t72, t79, t2235, t5399, t17635, t19334);
        let t96579 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2113(t1865, t26045, t26048, t26070, t26073, t26076, t27982, t6492, t6506, t6510, t7435, t7442, t96547, t96551, t96553, t96556, t96559, t96562);
        let t96605 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114(t26021, t26025, t26028, t26045, t26051, t26063, t26070, t26073, t26076, t27979, t6506, t6510, t7428, t7432, t7435, t7442, t7446, t90182, t90185);
        let t96649 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115(t2235, t5392, t16558, t17635, t17686, t17691, t1860, t1864, t1865, t22502, t22505, t26021, t26024, t26025, t26028, t26044, t26048, t27949, t27950, t27953, t27957, t5398, t6486, t6500, t6509, t67, t7428, t7441, t7445, t7446, t83791, t83796, t83803);
        let (t96654, t96655, t96657) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116(t5, t96409, t96441, t96478, t96509, t96545, t96579, t96605, t96649, t112, t5456, t6514, t19534, t88);
        let (t96659, t96661, t96663, t96665, t96667, t96669, t96671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117(t1873, t96657, t28007, t6534, t26114, t7467, t26117, t26135, t7676, t2314, t28017, t5113);
        let t96682 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118(t1873, t96356, t28002, t6534, t12725, t7467, t75560, t19451, t96654, t96655, t96659, t96661, t96663, t96665, t96667, t96669, t96671);
    (t96654, t96655, t96682)
}
