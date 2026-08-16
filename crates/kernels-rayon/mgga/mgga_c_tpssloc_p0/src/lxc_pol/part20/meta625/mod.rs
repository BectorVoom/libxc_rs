//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2249;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2251;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2253;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2255;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta625(t13210: f64, t9638: f64, t120: f64, t13170: f64, t2553: f64, t828: f64, t13231: f64, t13258: f64, t41107: f64, t4250: f64, t13244: f64, t242: f64, t812: f64, t841: f64, t10003: f64, t13222: f64, t13228: f64, t13229: f64, t13251: f64, t13300: f64, t13353: f64, t16935: f64, t2633: f64, t2643: f64, t2645: f64, t41025: f64, t41031: f64, t41467: f64, t4178: f64, t4180: f64, t4182: f64, t4248: f64, t829: f64, t9616: f64, t9642: f64, t1484: f64, t2678: f64, t41115: f64, t4166: f64, t9637: f64, t2649: f64, t13257: f64, t2617: f64, t4184: f64, t4257: f64, t9993: f64, t13176: f64, t2638: f64, t831: f64, t13350: f64, t2647: f64, t41048: f64, t41050: f64, t41053: f64, t41055: f64, t41063: f64, t4191: f64, t9623: f64, t9661: f64, t9990: f64, t13278: f64, t2681: f64, t4236: f64, t9674: f64, t13186: f64, t2697: f64, t13289: f64, t41011: f64, t4179: f64, t820: f64, t1509: f64, t13225: f64, t13177: f64, t13242: f64, t13254: f64, t13262: f64, t13263: f64, t1495: f64, t210: f64, t2686: f64, t40971: f64, t41161: f64, t4181: f64, t843: f64, t9458: f64, t13312: f64, t4240: f64, t13261: f64, t836: f64, t9972: f64, t13265: f64, t13333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46595, t46597, t46606, t46611, t46616, t46618, t46628) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2249(t13210, t9638, t120, t13170, t2553, t828, t13231, t13258, t41107, t4250, t13244, t242, t812, t841);
        let t46637 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2250(t10003, t13222, t13228, t13229, t13251, t13300, t13353, t16935, t2633, t2643, t2645, t41025, t41031, t41467, t4178, t4180, t4182, t4248, t46595, t46597, t46606, t46611, t46616, t46618, t46628, t829, t9616, t9642);
        let (t46644, t46650, t46658, t46661, t46663) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2251(t1484, t2678, t41115, t4250, t4166, t9637, t2649, t13257, t2617, t4184, t4257, t9993);
        let t46670 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2252(t13176, t2638, t831, t13251, t13350, t2643, t2645, t2647, t41048, t41050, t41053, t41055, t41063, t4191, t4248, t4257, t46644, t46650, t46658, t46661, t46663, t9623, t9661, t9990);
        let (t46675, t46677, t46679, t46686, t46692, t46693) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2253(t13278, t2681, t4236, t9674, t13186, t2697, t13289, t41011, t4179, t820, t1509, t2678);
        let t46716 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2254(t13225, t9638, t13177, t13222, t13231, t13242, t13254, t13262, t13263, t1484, t1495, t210, t2643, t2686, t40971, t41161, t4180, t4181, t46644, t46675, t46677, t46679, t46686, t46692, t46693, t820, t829, t843, t9458, t9642, t9661);
        let (t46717, t46733, t46737, t46742, t46748) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2255(t13312, t9638, t41107, t4240, t13261, t2617, t812, t836, t9972, t13265, t13258, t13333);
    (t46597, t46637, t46644, t46670, t46692, t46693, t46716, t46717, t46733, t46737, t46742, t46748)
}
