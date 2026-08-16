//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1307;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1308;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1309;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1310;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1311;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta370(t118: f64, t5527: f64, t794: f64, t9549: f64, t16662: f64, t210: f64, t214: f64, t5544: f64, t2576: f64, t2563: f64, t5555: f64, t213: f64, t221: f64, t776: f64, t13014: f64, t13020: f64, t13022: f64, t13027: f64, t4127: f64, t787: f64, t9579: f64, t9583: f64, t16781: f64, t225: f64, t10054: f64, t5585: f64, t13176: f64, t1499: f64, t1523: f64, t1525: f64, t16673: f64, t16679: f64, t16754: f64, t16756: f64, t16759: f64, t16762: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4286: f64, t4291: f64, t4296: f64, t4298: f64, t5645: f64, t5648: f64, t5653: f64, t812: f64, t861: f64, t252: f64, t5584: f64, t828: f64, t9975: f64, t16758: f64, t4182: f64, t2732: f64, t5617: f64, t829: f64, t4290: f64, t4177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16784, t16787, t16792, t16794, t16796) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1307(t118, t5527, t794, t9549, t16662, t210, t214, t5544, t2576, t2563, t5555, t213);
        let t16803 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1308(t16796, t221, t776, t13014, t13020, t13022, t13027, t16784, t16787, t16792, t16794, t4127, t787, t9579, t9583);
        let (t16804, t16805, t16814) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1309(t16781, t16803, t225, t10054, t5585, t13176, t1499, t1523, t1525, t16673, t16679, t16754, t16756, t16759, t16762, t255, t2617, t4162, t4166, t4286, t4291, t4296, t4298, t5645, t5648, t5653, t812, t861);
        let t16815 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1310(t252, t5584);
        let (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1311(t828, t9975, t16815, t16758, t4182, t2732, t5617, t829, t1499, t4290, t4166, t4177);
    (t16804, t16805, t16814, t16815, t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836)
}
