//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1778;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1779;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta360(t13360: f64, t849: f64, t13176: f64, t842: f64, t1516: f64, t9601: f64, t10012: f64, t10014: f64, t10026: f64, t10029: f64, t10030: f64, t10036: f64, t10038: f64, t13333: f64, t13337: f64, t13345: f64, t13347: f64, t13353: f64, t13359: f64, t249: f64, t2623: f64, t2643: f64, t2703: f64, t2707: f64, t4172: f64, t4178: f64, t4261: f64, t843: f64, t9990: f64, t13213: f64, t13268: f64, t13331: f64, t218: f64, t1509: f64, t852: f64, t829: f64, t252: f64, t4233: f64, t4182: f64, t2684: f64, t4282: f64, t4290: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13362, t13365, t13368, t13375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1778(t13360, t849, t13176, t842, t1516, t9601, t10012, t10014, t10026, t10029, t10030, t10036, t10038, t13333, t13337, t13345, t13347, t13353, t13359, t249, t2623, t2643, t2703, t2707, t4172, t4178, t4261, t843, t9990);
        let (t13377, t13378, t13380, t13381, t13384, t13385, t13388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1779(t13213, t13268, t13331, t13375, t218, t1509, t852, t829, t252, t4233, t4182, t2684, t4282);
        let t13390 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1780(t4290, t808);
    (t13362, t13365, t13368, t13377, t13378, t13380, t13381, t13384, t13385, t13388, t13390)
}
