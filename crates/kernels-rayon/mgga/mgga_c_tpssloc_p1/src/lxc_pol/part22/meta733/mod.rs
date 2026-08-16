//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta733 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2405;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2406;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta733(t13515: f64, t5727: f64, t17423: f64, t4354: f64, t49269: f64, t5730: f64, t21268: f64, t42143: f64, t21300: f64, t2787: f64, t47705: f64, t47707: f64, t48103: f64, t48919: f64, t48924: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t41684: f64, t41863: f64, t68460: f64, t68464: f64, t68468: f64, t68472: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t68509: f64, t68511: f64, t68515: f64, t68518: f64, t68523: f64, t68527: f64, t68530: f64, t48155: f64, t59657: f64, t60163: f64, t60168: f64, t60173: f64, t68536: f64, t68541: f64, t68545: f64, t68549: f64, t68552: f64, t68556: f64, t68563: f64, t48157: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t42086: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64, t42087: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t60274: f64, t68619: f64, t68626: f64, t68628: f64, t68630: f64, t68633: f64, t68635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68767, t68769, t68771, t68773, t68775, t68785) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404(t13515, t5727, t17423, t4354, t49269, t5730, t21268, t42143, t21300, t2787, t47705, t47707, t48103, t48919, t48924, t68442, t68444, t68446, t68448, t68452, t68454);
        let t68798 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2405(t41684, t41863, t68460, t68464, t68468, t68472, t68479, t68483, t68486, t68489, t68492, t68494);
        let t68812 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2406(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
        let t68825 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407(t48155, t59657, t60163, t60168, t60173, t68536, t68541, t68545, t68549, t68552, t68556, t68563);
        let (t68839, t68851) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408(t48157, t60192, t60194, t60202, t68571, t68577, t68580, t68583, t68586, t68589, t68592, t42086, t59663, t59665, t59680, t59688, t59694, t60204, t68596, t68599, t68602, t68605, t68608);
        let t68864 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409(t42087, t47787, t59700, t59702, t59704, t60274, t68619, t68626, t68628, t68630, t68633, t68635);
    (t68767, t68769, t68771, t68773, t68775, t68785, t68798, t68812, t68825, t68839, t68851, t68864)
}
