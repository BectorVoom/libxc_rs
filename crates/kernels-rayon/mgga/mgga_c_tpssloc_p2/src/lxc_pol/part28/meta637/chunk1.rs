//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2033/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2033(t12734: f64, t12813: f64, t1458: f64, t16148: f64, t16153: f64, t16503: f64, t1983: f64, t2040: f64, t2075: f64, t2079: f64, t2314: f64, t23909: f64, t23958: f64, t24028: f64, t24428: f64, t24987: f64, t24995: f64, t26114: f64, t26179: f64, t26559: f64, t27150: f64, t27226: f64, t4028: f64, t4034: f64, t4072: f64, t652: f64, t7050: f64, t7156: f64, t7170: f64, t7171: f64, t7685: f64, t7802: f64, t90023: f64, t9016: f64, t90370: f64, t91669: f64, t91753: f64) -> f64 {
    let t94022 = -4.0_f64 * t90370 * t2040 - 4.0_f64 * t26114 * t7050 - 2.0_f64 * t652 * t24428 * t1458 + 6.0_f64 * t24987 * t7171 + 3.0_f64 * t1983 * t7170 * t90023 + 6.0_f64 * t7685 * t23958 + 6.0_f64 * t24995 * t9016 * t16153 + 12.0_f64 * t24995 * t9016 * t16148 + 4.0_f64 * t91669 * t26559 - 4.0_f64 * t4034 * t27150 - 4.0_f64 * t652 * t7156 * t4072 - 2.0_f64 * t4028 * t23909 - 4.0_f64 * t12734 * t7802 - 4.0_f64 * t2314 * t27226 + t2079 * t16503 - 2.0_f64 * t7685 * t24028 - 2.0_f64 * t652 * t2075 * t12813 - 2.0_f64 * t91753 * t2040 - 4.0_f64 * t26179 * t7050;
    t94022
}
