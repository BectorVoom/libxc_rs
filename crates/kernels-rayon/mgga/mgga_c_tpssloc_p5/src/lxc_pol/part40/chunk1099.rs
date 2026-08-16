//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1099/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1099(t1569: f64, t4433: f64, t5762: f64, t931: f64, t5759: f64, t2888: f64, t5758: f64, t4437: f64, t10813: f64, t5742: f64, t10771: f64, t10811: f64, t14271: f64, t14276: f64, t17519: f64, t17523: f64, t17526: f64, t17530: f64, t17535: f64, t2861: f64, t2886: f64, t4416: f64, t4438: f64) -> f64 {
    let t17538 = t1569 * t4433;
    let t17541 = t5762 * t931;
    let t17544 = t5759 * t931;
    let t17547 = t5758 * t2888;
    let t17548 = t17547 * t931;
    let t17551 = t4437 * t4433;
    let t17554 = t5742 * t10813;
    let t17555 = t17554 * t931;
    let t17558 = t17519 - t17523 - t17526 - t17530 - 4.0_f64 * t14276 * t4416 + 0.64327917994770140268e2_f64 * t14271 * t4438 + 6.0_f64 * t2886 * t17535 - 4.0_f64 * t2861 * t17538 - 0.19298375398431042081e3_f64 * t10771 * t17541 - 2.0_f64 * t2861 * t17544 + 0.32163958997385070134e2_f64 * t2886 * t17548 + 0.64327917994770140268e2_f64 * t2886 * t17551 + 0.2069040516770936012e4_f64 * t10811 * t17555;
    t17558
}
