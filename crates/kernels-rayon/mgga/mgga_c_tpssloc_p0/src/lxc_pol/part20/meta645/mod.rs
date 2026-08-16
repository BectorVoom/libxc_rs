//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2371;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta645(t10216: f64, t13797: f64, t3067: f64, t353: f64, t373: f64, t383: f64, t1021: f64, t820: f64, t10482: f64, t1615: f64, t10390: f64, t10858: f64, t10883: f64, t13975: f64, t14069: f64, t14080: f64, t14211: f64, t2986: f64, t3039: f64, t3041: f64, t3057: f64, t3064: f64, t3121: f64, t42388: f64, t42397: f64, t42436: f64, t42460: f64, t42511: f64, t43235: f64, t43361: f64, t4575: f64, t4582: f64, t4593: f64, t45971: f64, t48265: f64, t1041: f64, t13969: f64, t14142: f64, t14179: f64, t10309: f64, t10408: f64, t14126: f64, t14167: f64, t1616: f64, t2776: f64, t3070: f64, t3071: f64, t3117: f64, t42478: f64, t42481: f64, t42490: f64, t42546: f64, t43358: f64, t4579: f64, t4650: f64, t47779: f64, t47915: f64, t48260: f64, t48497: f64, t10375: f64, t1612: f64, t1539: f64, t248: f64, t42749: f64, t14473: f64, t2952: f64, t10633: f64, t4483: f64, t47705: f64, t47707: f64, t47730: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47732: f64, t47736: f64, t47738: f64, t41655: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41713: f64, t47744: f64, t47748: f64, t47761: f64, t47765: f64, t47769: f64, t47777: f64, t47781: f64, t47785: f64, t47787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48607, t48611, t48612, t48622) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368(t10216, t13797, t3067, t353, t373, t383, t1021, t820, t10482, t1615, t10390, t10858, t10883, t13975, t14069, t14080, t14211, t2986, t3039, t3041, t3057, t3064, t3121, t42388, t42397, t42436, t42460, t42511, t43235, t43361, t4575, t4582, t4593, t45971, t48265);
        let t48656 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369(t1041, t13969, t14142, t14179, t10309, t10408, t14126, t14167, t1616, t2776, t3070, t3071, t3117, t42478, t42481, t42490, t42546, t43358, t4579, t4582, t4650, t47779, t47915, t48260, t48497, t48607);
        let (t48670, t48674, t48679, t48681, t48688, t48689) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370(t10375, t1612, t1041, t1539, t248, t42749, t14473, t2952, t10633, t4483, t47705, t47707);
        let t48702 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2371(t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738, t48688, t48689);
        let t48722 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2372(t41655, t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
    (t48611, t48612, t48622, t48656, t48670, t48674, t48679, t48681, t48702, t48722)
}
