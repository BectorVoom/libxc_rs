//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1105/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1105(t825: f64, t826: f64, t9829: f64, t15362: f64, t9810: f64, t28126: f64, t5841: f64, t7810: f64, t23296: f64, t787: f64, t9824: f64, t549: f64, t6111: f64, t7292: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28795 = t825 * t826 * t9829;
    let t28800 = 0.11916829983950142223e0_f64 * t15362 * t9810;
    let t28810 = 0.38342925953920749676e1_f64 * t7810 * t5841 * t28126;
    let t28811 = t787 * t23296;
    let t28813 = 0.59584149919750711116e-1_f64 * t28811 * t9824;
    let t28816 = 0.23833659967900284446e0_f64 * t6111 * t549 * t7292;
    (t28795, t28800, t28810, t28811, t28813, t28816)
}
