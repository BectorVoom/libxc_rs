//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta724 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2320;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta724(t18329: f64, t7310: f64, t1734: f64, t18303: f64, t18948: f64, t18955: f64, t19058: f64, t19062: f64, t19072: f64, t19077: f64, t24729: f64, t24733: f64, t27604: f64, t27617: f64, t478: f64, t4974: f64, t4980: f64, t4984: f64, t4989: f64, t7345: f64, t7376: f64, t86146: f64, t86171: f64, t95270: f64, t95273: f64, t95303: f64, t95304: f64, t18324: f64, t18371: f64, t24741: f64, t29569: f64, t29651: f64, t4954: f64, t7321: f64, t86184: f64, t95320: f64, t95334: f64, t95335: f64, t95352: f64, t95362: f64, t95364: f64, t95365: f64, t95687: f64, t19031: f64, t2139: f64, t471: f64, t24746: f64, t27607: f64, t8027: f64, t1409: f64, t1714: f64, t2132: f64, t52: f64, t6138: f64, t1222: f64, t29597: f64, t29601: f64, t1232: f64, t18383: f64, t18965: f64, t2136: f64, t24736: f64, t25588: f64, t29625: f64, t6207: f64, t7316: f64, t86191: f64, t86327: f64, t95370: f64, t1193: f64, t29585: f64, t29562: f64, t27674: f64, t5040: f64, t18368: f64, t27629: f64, t27692: f64, t8040: f64, t86324: f64, t95384: f64, t95404: f64, t95410: f64, t95424: f64, t95435: f64, t95566: f64, t95678: f64, t29643: f64, t3503: f64, t86264: f64, t1210: f64, t29647: f64, t95332: f64, t29561: f64, t6739: f64, t7325: f64, t1215: f64, t15394: f64, t18206: f64, t18211: f64, t18232: f64, t18573: f64, t2121: f64, t2140: f64, t24821: f64, t27636: f64, t27642: f64, t27697: f64, t488: f64, t4899: f64, t5011: f64, t6224: f64, t7331: f64, t7999: f64, t85972: f64, t95396: f64, t95446: f64, t95450: f64, t1202: f64, t2133: f64, t24815: f64, t27637: f64, t27655: f64, t27704: f64, t29600: f64, t29615: f64, t29644: f64, t29648: f64, t4950: f64, t6144: f64, t8028: f64, t86149: f64, t95456: f64, t95459: f64, t95463: f64, t95465: f64, t99767: f64, t27628: f64, t95648: f64, t24682: f64, t460: f64, t27635: f64, t3: f64, t95326: f64, t11716: f64, t24685: f64, t27638: f64, t27639: f64, t27644: f64, t27645: f64, t29594: f64, t6218: f64, t85966: f64, t86234: f64, t95415: f64, t95649: f64, t27634: f64, t3030: f64, t95420: f64, t18387: f64, t18969: f64, t29563: f64, t3032: f64, t475: f64, t4965: f64, t8048: f64, t86275: f64, t86278: f64, t95480: f64, t95487: f64, t95491: f64, t18356: f64, t27614: f64, t4997: f64, t1730: f64, t27603: f64, t27598: f64, t5001: f64, t1218: f64, t1737: f64, t18523: f64, t19101: f64, t2134: f64, t5014: f64, t6211: f64, t6227: f64, t7320: f64, t86140: f64, t95238: f64, t95507: f64, t95511: f64, t95512: f64, t18221: f64, t18225: f64, t18237: f64, t18940: f64, t24650: f64, t3448: f64, t6729: f64, t68: f64, t7326: f64, t7328: f64, t7573: f64, t95340: f64, t95346: f64, t95387: f64, t95515: f64, t95517: f64, t95520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t104087 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318(t18329, t7310, t1734, t18303, t18948, t18955, t19058, t19062, t19072, t19077, t24729, t24733, t27604, t27617, t478, t4974, t4980, t4984, t4989, t7345, t7376, t86146, t86171, t95270, t95273, t95303, t95304);
        let t104101 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319(t18324, t7310, t18371, t24741, t29569, t29651, t4954, t7321, t86184, t95320, t95334, t95335, t95352, t95362, t95364, t95365, t95687);
        let (t104107, t104111, t104118, t104120, t104122, t104124, t104126) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2320(t19031, t2139, t471, t24746, t27607, t8027, t1409, t1714, t2132, t52, t6138, t1222, t29597);
        let t104134 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321(t1222, t29601, t104107, t104111, t104120, t104124, t104126, t1232, t18383, t18965, t2136, t24736, t24741, t25588, t29625, t6207, t7316, t8027, t86191, t86327, t95370);
        let t104155 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322(t1193, t29585, t2136, t29562, t52, t27674, t5040, t1409, t8027, t18368, t27629, t27692, t4954, t8040, t86324, t95384, t95404, t95410, t95424, t95435, t95566, t95678);
        let t104193 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323(t29643, t3503, t86264, t1210, t29647, t8040, t95332, t29561, t6739, t7325, t1215, t15394, t18206, t18211, t18232, t18573, t2121, t2140, t24821, t27636, t27642, t27697, t488, t4899, t5011, t6224, t7331, t7999, t85972, t95396, t95446, t95450);
        let t104220 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324(t1202, t2132, t2133, t2136, t24815, t27636, t27637, t27655, t27704, t29600, t29615, t29644, t29648, t488, t4950, t5011, t6144, t7316, t7321, t8028, t86149, t95456, t95459, t95463, t95465, t95687, t99767);
        let t104264 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325(t27628, t95648, t104118, t24682, t460, t104122, t27635, t3, t95326, t11716, t1210, t1215, t24685, t27636, t27638, t27639, t27644, t27645, t29594, t29644, t29648, t3503, t6218, t6224, t7331, t8040, t85966, t86234, t95396, t95415, t95649);
        let (t104280, t104292) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326(t27634, t3030, t95420, t52, t6144, t24682, t460, t1210, t1215, t18387, t18969, t24741, t27639, t27645, t29563, t3032, t475, t488, t4965, t6224, t7321, t7331, t8048, t86275, t86278, t95396, t95480, t95487, t95491);
        let t104319 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327(t18356, t24729, t27614, t4997, t1730, t27603, t27598, t5001, t1218, t1232, t1737, t18523, t19101, t2134, t24736, t460, t5014, t6211, t6227, t7320, t7345, t86140, t95238, t95507, t95511, t95512);
        let t104351 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328(t104280, t2132, t24746, t1714, t18221, t18225, t18237, t18940, t2121, t2136, t24650, t29562, t29594, t3448, t475, t6729, t68, t7321, t7326, t7328, t7573, t95340, t95346, t95387, t95515, t95517, t95520);
    (t104087, t104101, t104134, t104155, t104193, t104220, t104264, t104292, t104319, t104351)
}
