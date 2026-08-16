//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta682 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2304;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2309;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2312;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta682(t24574: f64, t27462: f64, t1185: f64, t86036: f64, t974: f64, t3030: f64, t460: f64, t27488: f64, t27491: f64, t27495: f64, t27497: f64, t1170: f64, t2121: f64, t27732: f64, t11881: f64, t15000: f64, t1653: f64, t1716: f64, t24778: f64, t24795: f64, t24829: f64, t27406: f64, t27531: f64, t3243: f64, t4964: f64, t7283: f64, t7362: f64, t7373: f64, t7376: f64, t7389: f64, t8073: f64, t8082: f64, t85814: f64, t85947: f64, t86076: f64, t15590: f64, t7338: f64, t27614: f64, t3572: f64, t27617: f64, t3523: f64, t1218: f64, t15531: f64, t15535: f64, t15622: f64, t15627: f64, t15631: f64, t15637: f64, t24729: f64, t24733: f64, t4984: f64, t86120: f64, t86146: f64, t86164: f64, t86167: f64, t86171: f64, t15437: f64, t24728: f64, t24732: f64, t4965: f64, t7344: f64, t1232: f64, t1737: f64, t27604: f64, t3496: f64, t3511: f64, t3518: f64, t3527: f64, t3531: f64, t86122: f64, t86124: f64, t86126: f64, t86136: f64, t24658: f64, t27683: f64, t1184: f64, t24682: f64, t27607: f64, t1209: f64, t85821: f64, t1215: f64, t15555: f64, t15612: f64, t15650: f64, t15704: f64, t24655: f64, t24664: f64, t24670: f64, t24716: f64, t24736: f64, t27684: f64, t478: f64, t4974: f64, t4980: f64, t5014: f64, t7345: f64, t86140: f64, t86327: f64, t15743: f64, t24649: f64, t27710: f64, t23508: f64, t8026: f64, t7325: f64, t27628: f64, t7324: f64, t7331: f64, t15730: f64, t7339: f64, t24661: f64, t15617: f64, t27711: f64, t86174: f64, t86176: f64, t86184: f64, t86234: f64, t24668: f64, t15643: f64, t27639: f64, t86264: f64, t27645: f64, t3540: f64, t8043: f64, t15545: f64, t15667: f64, t24699: f64, t24749: f64, t27655: f64, t7310: f64, t7316: f64, t8028: f64, t8035: f64, t86191: f64, t2136: f64, t607: f64, t8027: f64, t1714: f64, t27634: f64, t10469: f64, t24719: f64, t3: f64, t86154: f64, t2132: f64, t24746: f64, t24685: f64, t27629: f64, t27636: f64, t27638: f64, t27642: f64, t27692: f64, t3032: f64, t3503: f64, t3507: f64, t3566: f64, t475: f64, t488: f64, t4954: f64, t5011: f64, t8040: f64, t8048: f64, t86199: f64, t86330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95192, t95194, t95197, t95201, t95213) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2304(t24574, t27462, t1185, t86036, t974, t3030, t460, t27488, t27491, t27495, t27497, t1170, t2121, t27732);
        let t95224 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305(t11881, t15000, t1653, t1716, t24778, t24795, t24829, t27406, t27531, t3243, t4964, t7283, t7362, t7373, t7376, t7389, t8073, t8082, t85814, t85947, t86076, t95192, t95194, t95197, t95201, t95213);
        let t95260 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306(t15590, t7338, t27614, t3572, t27617, t3523, t1218, t15531, t15535, t15622, t15627, t15631, t15637, t24729, t24733, t4984, t86120, t86146, t86164, t86167, t86171);
        let t95285 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307(t15437, t24728, t24732, t4965, t7344, t1232, t1737, t27604, t27614, t27617, t3496, t3511, t3518, t3527, t3531, t86122, t86124, t86126, t86136);
        let t95316 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308(t24658, t27683, t1184, t24682, t27607, t1209, t85821, t1215, t15555, t15612, t15650, t15704, t24655, t24664, t24670, t24716, t24729, t24736, t27684, t478, t4974, t4980, t5014, t7345, t7376, t86140, t86327);
        let (t95320, t95323, t95327, t95334, t95335) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2309(t15743, t7345, t24649, t27710, t23508, t8026, t7325, t27628, t7324, t7331, t15730, t7339);
        let t95343 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310(t24661, t27491, t15617, t24655, t24664, t24670, t27711, t7331, t7345, t86174, t86176, t86184, t86234, t95320, t95323, t95327, t95334, t95335);
        let t95367 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311(t24668, t27497, t15643, t7345, t27639, t86264, t27645, t3540, t8043, t15545, t15667, t24699, t24749, t27655, t7310, t7316, t8028, t8035, t86191, t86234);
        let (t95370, t95382, t95384, t95387, t95396) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2312(t2136, t607, t8027, t1714, t24682, t460, t27628, t27634, t10469, t24719, t3, t86154);
        let t95407 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313(t2132, t24746, t95382, t24655, t24664, t24670, t24685, t27629, t27636, t27638, t27642, t27692, t3032, t3503, t3507, t3566, t475, t488, t4954, t5011, t7331, t8040, t8048, t86199, t86330, t95370, t95384, t95387, t95396);
    (t95224, t95260, t95285, t95316, t95343, t95367, t95396, t95407)
}
