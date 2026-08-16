//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta683 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2316;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2318;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2320;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2321;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2322;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta683(t1222: f64, t27589: f64, t1184: f64, t1409: f64, t24682: f64, t460: f64, t461: f64, t1009: f64, t7324: f64, t24722: f64, t15548: f64, t24733: f64, t27598: f64, t3535: f64, t1218: f64, t14731: f64, t14736: f64, t14740: f64, t15663: f64, t15750: f64, t2121: f64, t24736: f64, t24741: f64, t4899: f64, t4989: f64, t7331: f64, t8040: f64, t86204: f64, t86324: f64, t2132: f64, t24746: f64, t3545: f64, t8020: f64, t1202: f64, t27603: f64, t4993: f64, t15486: f64, t7345: f64, t27599: f64, t3572: f64, t27604: f64, t3523: f64, t1232: f64, t1748: f64, t2136: f64, t3587: f64, t86129: f64, t86228: f64, t86248: f64, t88391: f64, t24683: f64, t8027: f64, t4928: f64, t52: f64, t86292: f64, t15564: f64, t23413: f64, t86262: f64, t86266: f64, t86269: f64, t86273: f64, t86275: f64, t86278: f64, t86327: f64, t15689: f64, t7310: f64, t27674: f64, t3548: f64, t15753: f64, t27608: f64, t7321: f64, t27586: f64, t15357: f64, t15560: f64, t2134: f64, t24650: f64, t27580: f64, t27692: f64, t27714: f64, t7320: f64, t86282: f64, t86296: f64, t3540: f64, t8049: f64, t3966: f64, t24716: f64, t4997: f64, t15459: f64, t15463: f64, t15470: f64, t15710: f64, t24706: f64, t3562: f64, t5030: f64, t8031: f64, t86293: f64, t86299: f64, t15492: f64, t7339: f64, t15734: f64, t25588: f64, t1244: f64, t1742: f64, t3068: f64, t1210: f64, t24721: f64, t27691: f64, sigma2: f64, t27700: f64, t86261: f64, t15239: f64, t15474: f64, t15541: f64, t15761: f64, t27617: f64, t3580: f64, t475: f64, t68: f64, t7326: f64, t7328: f64, t86313: f64, t15418: f64, t4724: f64, t24720: f64, t27710: f64, t11588: f64, t4729: f64, t14749: f64, t14753: f64, t15455: f64, t15764: f64, t2140: f64, t3448: f64, t488: f64, t86341: f64, t86343: f64, t86348: f64, t86350: f64, t15572: f64, t15501: f64, t24727: f64, t3500: f64, t7337: f64, t15478: f64, t15527: f64, t15656: f64, t15714: f64, t24699: f64, t24815: f64, t27636: f64, t27637: f64, t3493: f64, t3496: f64, t3511: f64, t3518: f64, t8028: f64, t86354: f64, t3: f64, t24684: f64, t15608: f64, t11716: f64, t14744: f64, t1714: f64, t1734: f64, t27642: f64, t27644: f64, t27704: f64, t3507: f64, t4950: f64, t5011: f64, t6729: f64, t85827: f64, t85966: f64, t85972: f64, t86194: f64, t86330: f64, t86357: f64, t95396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95410, t95413, t95415, t95424, t95435) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2314(t1222, t27589, t1184, t1409, t24682, t460, t461, t1009, t7324, t24722, t15548, t24733);
        let t95443 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315(t27598, t3535, t1218, t14731, t14736, t14740, t15663, t15750, t2121, t24736, t24741, t4899, t4989, t7331, t8040, t86204, t86324, t95410, t95415, t95424, t95435);
        let (t95446, t95450, t95452, t95456, t95459, t95463) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2316(t2132, t24746, t95413, t3545, t8020, t1202, t27603, t24736, t4993, t15486, t7345, t27599, t3572);
        let t95469 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317(t27604, t3523, t1232, t1748, t2132, t2136, t3587, t86129, t86228, t86248, t88391, t95446, t95450, t95452, t95456, t95459, t95463);
        let (t95484, t95492) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2318(t24683, t24746, t8027, t4928, t52, t2132, t8040, t86292, t15564, t2136, t23413, t86262, t86266, t86269, t86273, t86275, t86278, t86327);
        let t95518 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319(t15689, t7310, t27674, t3548, t15753, t27608, t7321, t1222, t27586, t15357, t15560, t2134, t24650, t27580, t27692, t27714, t460, t7320, t8040, t86282, t86296, t86324);
        let t95543 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2320(t3540, t8049, t2132, t2136, t3966, t24716, t4997, t15459, t15463, t15470, t15710, t24706, t24736, t24741, t27674, t3562, t5030, t8031, t86293, t86299);
        let (t95545, t95550, t95556, t95566, t95571) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2321(t15492, t7339, t15734, t7345, t25588, t461, t7324, t1244, t1742, t3068, t1210, t24721, t27691, sigma2);
        let t95576 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2322(t27700, t86261, t15239, t15474, t15541, t15761, t24741, t27617, t3580, t3587, t475, t68, t7326, t7328, t7331, t7345, t86313, t95545, t95550, t95556, t95566, t95571);
        let t95603 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323(t15418, t2121, t4724, t24720, t27710, t24722, t11588, t4729, t14749, t14753, t15455, t15764, t2140, t3448, t488, t7345, t86341, t86343, t86348, t86350);
        let t95633 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324(t15572, t24741, t15501, t24727, t3500, t7337, t15478, t15527, t15656, t15714, t24699, t24706, t24815, t27599, t27636, t27637, t3493, t3496, t3511, t3518, t7339, t7345, t8028, t8031, t86354);
        let t95672 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325(t27710, t3, t24684, t15608, t24741, t11716, t1210, t14744, t1714, t1734, t2121, t2132, t24699, t27636, t27637, t27642, t27644, t27704, t3448, t3507, t475, t4950, t5011, t6729, t7321, t7331, t8040, t85827, t85966, t85972, t86194, t86330, t86357, t95396);
    (t95443, t95469, t95484, t95492, t95518, t95543, t95576, t95603, t95633, t95672)
}
