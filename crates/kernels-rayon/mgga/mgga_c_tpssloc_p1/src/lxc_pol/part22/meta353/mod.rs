//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1566;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta353(t17030: f64, t4182: f64, t1499: f64, t4280: f64, t16935: f64, t4282: f64, t13433: f64, t1510: f64, t829: f64, t13397: f64, t16817: f64, t16820: f64, t16823: f64, t16825: f64, t16828: f64, t16830: f64, t17023: f64, t17028: f64, t226: f64, t2617: f64, t4166: f64, t4281: f64, t4283: f64, t4288: f64, t4291: f64, t4292: f64, t5575: f64, t5651: f64, t5655: f64, t808: f64, t812: f64, t863: f64, t16814: f64, t858: f64, t225: f64, t5559: f64, t5657: f64, t865: f64, t2718: f64, t17022: f64, t218: f64, t5636: f64, t10110: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17031, t17034, t17037, t17041, t17046, t17048) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1566(t17030, t4182, t1499, t4280, t16935, t4282, t13433, t1510, t829, t13397, t16817, t16820, t16823, t16825, t16828, t16830, t17023, t17028, t226, t2617, t4166, t4281, t4283, t4288, t4291, t4292, t5575, t5651, t5655, t808, t812, t863);
        let (t17049, t17050, t17052, t17056, t17057, t17060, t17064) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1567(t16814, t17048, t858, t225, t5559, t5657, t865, t2718, t17022, t218, t5636, t10110);
    (t17031, t17034, t17037, t17041, t17046, t17049, t17050, t17052, t17056, t17057, t17060, t17064)
}
