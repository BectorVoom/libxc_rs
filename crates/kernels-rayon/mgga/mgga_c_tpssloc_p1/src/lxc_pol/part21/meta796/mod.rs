//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta796 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2760;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2761;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta796(t40817: f64, t13191: f64, t13487: f64, t16592: f64, t16606: f64, t17120: f64, t1877: f64, t193: f64, t2378: f64, t2522: f64, t2553: f64, t2749: f64, t39549: f64, t39563: f64, t40772: f64, t4307: f64, t4310: f64, t4314: f64, t5664: f64, t58071: f64, t58080: f64, t58085: f64, t58090: f64, t40: f64, t12606: f64, t12652: f64, t1430: f64, t16558: f64, t16637: f64, t16642: f64, t2244: f64, t2250: f64, t4104: f64, t5433: f64, t5435: f64, t55677: f64, t55723: f64, t607: f64, t75: f64, t767: f64, zeta_threshold: f64, t52: f64, t1431: f64, t16649: f64, t16654: f64, t4111: f64, t5437: f64, t5439: f64, t771: f64, t78: f64, t17083: f64, t225: f64, t5584: f64, t852: f64, t16805: f64, t68: f64, t10076: f64, t13171: f64, t13263: f64, t13381: f64, t13388: f64, t13390: f64, t13397: f64, t13456: f64, t16758: f64, t16816: f64, t16830: f64, t17030: f64, t17046: f64, t2633: f64, t4162: f64, t4281: f64, t4282: f64, t4290: f64, t4291: f64, t4292: f64, t4295: f64, t5612: f64, t812: f64, t861: f64, t1509: f64, t4265: f64, t13336: f64, t13393: f64, t13450: f64, t13453: f64, t1510: f64, t1525: f64, t16756: f64, t16815: f64, t16817: f64, t16820: f64, t16825: f64, t17031: f64, t17034: f64, t2617: f64, t2679: f64, t2684: f64, t47395: f64, t47419: f64, t5651: f64, t829: f64, t9612: f64, t1519: f64, t4233: f64, t2631: f64, t40933: f64, t13433: f64, t16828: f64, t17023: f64, t2613: f64, t4234: f64, t47386: f64, t5655: f64, t808: f64, t9632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58094, t58095) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2758(t40817, t13191, t13487, t16592, t16606, t17120, t1877, t193, t2378, t2522, t2553, t2749, t39549, t39563, t40772, t4307, t4310, t4314, t5664, t58071, t58080, t58085, t58090);
        let t58116 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759(t40, t12606, t12652, t1430, t16558, t16637, t16642, t2244, t2250, t4104, t5433, t5435, t55677, t55723, t607, t75, t767, zeta_threshold);
        let t58137 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2760(t52, t12606, t12652, t1431, t16558, t16649, t16654, t2244, t2250, t4111, t5437, t5439, t55677, t55723, t607, t771, t78, zeta_threshold);
        let t58139 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2761(t58116, t58137);
        let (t58143, t58166, t58181, t58194) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2762(t17083, t225, t5584, t852, t16805, t68, t10076, t13171, t13263, t13381, t13388, t13390, t13397, t13456, t16758, t16816, t16830, t17030, t17046, t2633, t4162, t4281, t4282, t4290, t4291, t4292, t4295, t5612, t812, t861);
        let (t58204, t58224) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2763(t1509, t4265, t13336, t13393, t13450, t13453, t1510, t1525, t16756, t16758, t16815, t16817, t16820, t16825, t16830, t17031, t17034, t2617, t2679, t2684, t4291, t47395, t47419, t5651, t812, t829, t9612);
        let (t58226, t58246, t58261) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2764(t1519, t4233, t2631, t40933, t13263, t13390, t13397, t13433, t16758, t16815, t16828, t17023, t17030, t2613, t2633, t2679, t2684, t4234, t4281, t4291, t47386, t5655, t58166, t808, t812, t829, t9632);
    (t58094, t58095, t58139, t58143, t58166, t58181, t58194, t58204, t58224, t58226, t58246, t58261)
}
