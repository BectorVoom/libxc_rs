//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta703 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta703(t1036: f64, t28572: f64, t1015: f64, t1022: f64, t17841: f64, t1935: f64, t23564: f64, t23604: f64, t25645: f64, t25652: f64, t25658: f64, t25679: f64, t28558: f64, t28582: f64, t28587: f64, t3032: f64, t343: f64, t360: f64, t4649: f64, t5872: f64, t6730: f64, t6734: f64, t7583: f64, t82911: f64, t88341: f64, t88362: f64, t88367: f64, t88385: f64, t88537: f64, t17624: f64, t6717: f64, t1933: f64, t1937: f64, t5398: f64, t10475: f64, t17738: f64, t23422: f64, t23678: f64, t25609: f64, t25653: f64, t25654: f64, t28578: f64, t3128: f64, t5866: f64, t5885: f64, t7574: f64, t82516: f64, t82542: f64, t88286: f64, t88415: f64, t40: f64, t5842: f64, t23479: f64, t17701: f64, t17877: f64, t18021: f64, t1941: f64, t23419: f64, t28525: f64, t378: f64, t4579: f64, t6722: f64, t83117: f64, t83215: f64, t88422: f64, t88425: f64, t88428: f64, t88440: f64, t88453: f64, t88513: f64, t1409: f64, t1597: f64, t23562: f64, t5836: f64, t18041: f64, t17649: f64, t17998: f64, t6747: f64, t83025: f64, t83028: f64, t88348: f64, t88479: f64, t88488: f64, t17611: f64, t6755: f64, t1934: f64, t17659: f64, t6765: f64, t17178: f64, t17183: f64, t18036: f64, t1920: f64, t23437: f64, t23529: f64, t2987: f64, t4509: f64, t5857: f64, t5869: f64, t5909: f64, t6735: f64, t7573: f64, t83016: f64, t83220: f64, t88503: f64, t88517: f64, t344: f64, t6740: f64, t5904: f64, t6764: f64, t1046: f64, t17681: f64, t17890: f64, t23483: f64, t23544: f64, t28526: f64, t5861: f64, t83121: f64, t88548: f64, t13797: f64, t17152: f64, t17161: f64, t17920: f64, t18016: f64, t18025: f64, t25585: f64, t25601: f64, t6758: f64, t7578: f64, t83080: f64, t88566: f64, t88569: f64, t16558: f64, t3: f64, t17677: f64, t17705: f64, t88575: f64, t88577: f64, t88582: f64, t88604: f64, t88622: f64, t88625: f64, t88636: f64, t88645: f64, t23472: f64, t28586: f64, t17615: f64, t17620: f64, t28566: f64, t5890: f64, t5894: f64, t6723: f64, t83008: f64, t88648: f64, t88689: f64, t88692: f64, t28581: f64, t82895: f64, t28577: f64, t25641: f64, t88451: f64, t1615: f64, t17157: f64, t17167: f64, t17171: f64, t25683: f64, t363: f64, t6800: f64, t88351: f64, t88354: f64, t88372: f64, t88430: f64, t88431: f64, t88704: f64, t88383: f64, t25650: f64, t3030: f64, t88449: f64, t17714: f64, t17959: f64, t23489: f64, t23537: f64, t25655: f64, t25660: f64, t25661: f64, t6742: f64, t6744: f64, t68: f64, t83157: f64, t88290: f64, t88407: f64, t88723: f64, t99492: f64, t99514: f64, t99535: f64, t99556: f64, t99571: f64) -> f64 {
        let t99600 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285(t1036, t28572, t1015, t1022, t17841, t1935, t23564, t23604, t25645, t25652, t25658, t25679, t28558, t28582, t28587, t3032, t343, t360, t4649, t5872, t6730, t6734, t7583, t82911, t88341, t88362, t88367, t88385, t88537);
        let t99635 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286(t17624, t6717, t1933, t1937, t5398, t1022, t10475, t17738, t23422, t23678, t25609, t25652, t25653, t25654, t28578, t3128, t4649, t5866, t5872, t5885, t7574, t7583, t82516, t82542, t82911, t88286, t88415, t88537);
        let (t99645, t99654) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287(t40, t5842, t1933, t23479, t17701, t17877, t18021, t1937, t1941, t23419, t28525, t28582, t378, t4579, t6722, t83117, t83215, t88422, t88425, t88428, t88440, t88453, t88513);
        let (t99660, t99665, t99682) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288(t1409, t1597, t23562, t343, t40, t5836, t99645, t18041, t23419, t17649, t17998, t6747, t7583, t83025, t83028, t88348, t88479, t88488);
        let t99709 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289(t17611, t6755, t1933, t1934, t5836, t17659, t6765, t1597, t17178, t17183, t18036, t1920, t23437, t23529, t2987, t4509, t5857, t5869, t5909, t6735, t7573, t83016, t83220, t88503, t88517);
        let t99736 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290(t28525, t344, t6740, t5904, t6764, t1046, t17681, t17890, t23419, t23483, t23544, t28526, t28578, t28582, t28587, t5857, t5861, t6735, t6747, t6765, t83117, t83121, t88548);
        let t99760 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291(t13797, t17152, t17161, t17920, t18016, t18025, t1920, t1933, t1934, t23419, t25585, t25601, t25609, t378, t4509, t5842, t5904, t6735, t6758, t7578, t83016, t83080, t88566, t88569);
        let t99772 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292(t16558, t3, t17677, t17705, t1933, t1937, t23419, t88575, t88577, t88582, t88604, t88622, t88625, t88636, t88645);
        let t99793 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293(t1933, t23479, t99665, t1015, t23472, t28586, t17615, t6717, t17620, t23422, t28558, t28566, t5890, t5894, t5909, t6723, t83008, t88648, t88689, t88692);
        let t99826 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294(t1933, t23479, t99660, t1015, t28581, t82895, t28577, t3128, t25641, t88451, t1615, t17157, t17167, t17171, t1920, t25679, t25683, t2987, t363, t4509, t6800, t88351, t88354, t88372, t88430, t88431, t88704);
        let t99855 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295(t7583, t88383, t25650, t3030, t88449, t1015, t17714, t17959, t23489, t23537, t25652, t25655, t25660, t25661, t28566, t28587, t360, t5866, t6730, t6742, t6744, t68, t83157, t88290, t88407, t88723);
        let t99859 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2296(t99492, t99514, t99535, t99556, t99571, t99600, t99635, t99654, t99682, t99709, t99736, t99760, t99772, t99793, t99826, t99855);
    t99859
}
