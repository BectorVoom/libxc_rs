//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1550;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1551;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta346(t16781: f64, t16803: f64, t225: f64, t10054: f64, t5585: f64, t13176: f64, t1499: f64, t1523: f64, t1525: f64, t16673: f64, t16679: f64, t16754: f64, t16756: f64, t16759: f64, t16762: f64, t255: f64, t2617: f64, t4162: f64, t4166: f64, t4286: f64, t4291: f64, t4296: f64, t4298: f64, t5645: f64, t5648: f64, t5653: f64, t812: f64, t861: f64, t252: f64, t5584: f64, t828: f64, t9975: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t16804, t16805, t16811, t16814) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1550(t16781, t16803, t225, t10054, t5585, t13176, t1499, t1523, t1525, t16673, t16679, t16754, t16756, t16759, t16762, t255, t2617, t4162, t4166, t4286, t4291, t4296, t4298, t5645, t5648, t5653, t812, t861);
        let t16815 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1551(t252, t5584);
        let t16816 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1552(t828, t9975);
    (t16804, t16805, t16811, t16814, t16815, t16816)
}
