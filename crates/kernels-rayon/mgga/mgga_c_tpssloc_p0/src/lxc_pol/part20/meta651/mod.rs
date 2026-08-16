//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta651(t13654: f64, t2842: f64, t2844: f64, t912: f64, t10727: f64, t13727: f64, t10731: f64, t13520: f64, t41811: f64, t4359: f64, t41623: f64, t4400: f64, t10817: f64, t14379: f64, t10655: f64, t14389: f64, t13655: f64, t2792: f64, t2904: f64, t4446: f64, t10523: f64, t1573: f64, t10629: f64, t10750: f64, t10757: f64, t10820: f64, t10829: f64, t14337: f64, t14344: f64, t1581: f64, t2900: f64, t2907: f64, t42106: f64, t4472: f64, t47681: f64, t47686: f64, t47691: f64, t47695: f64, t47699: f64, t47703: f64, t47705: f64, t48085: f64, t48087: f64, t48090: f64, t48092: f64, t48096: f64, t41831: f64, t41833: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47722: f64, t47724: f64, t47728: f64, t47730: f64, t41656: f64, t41658: f64, t41660: f64, t47732: f64, t47736: f64, t47738: f64, t47744: f64, t47748: f64, t48098: f64, t48101: f64, t48103: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49080, t49082, t49084, t49086, t49088) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394(t13654, t2842, t2844, t912, t10727, t13727, t10731, t13520, t41811, t4359, t41623, t4400);
        let (t49090, t49092, t49095, t49096, t49099, t49104) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395(t10817, t14379, t10655, t14389, t13655, t2792, t912, t2904, t4446, t10523, t1573, t10629);
        let t49113 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396(t10750, t10757, t10820, t10829, t14337, t14344, t1581, t2900, t2907, t42106, t4472, t49080, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49096, t49099, t49104);
        let (t49127, t49139) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397(t47681, t47686, t47691, t47695, t47699, t47703, t47705, t48085, t48087, t48090, t48092, t48096);
        let t49140 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2398(t41831, t41833, t47707, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t49139);
        let t49154 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2399(t47730, t41656, t41658, t41660, t47732, t47736, t47738, t47744, t47748, t48098, t48101, t48103);
    (t49080, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49113, t49127, t49140, t49154)
}
