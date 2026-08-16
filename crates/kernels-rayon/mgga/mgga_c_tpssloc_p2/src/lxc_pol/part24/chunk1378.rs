//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1378/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1378(t23637: f64, t82822: f64, t1920: f64, t23620: f64, t968: f64, t23617: f64, t6680: f64, t11034: f64, t11046: f64, t11048: f64, t11051: f64, t1950: f64, t23654: f64, t23701: f64, t23704: f64, t3180: f64, t3186: f64, t3200: f64, t4673: f64, t4684: f64, t6790: f64, t6811: f64, t82382: f64, t82730: f64, t82799: f64, t82803: f64, t82806: f64, t82809: f64) -> f64 {
    let t82823 = t82822 * t23637;
    let t82828 = t1920 * t968 * t23620;
    let t82830 = t6680 * t23617;
    let t82834 = t82799 - 0.24125699647107321069e0_f64 * t82382 * t6790 - 0.3752886611772249944e0_f64 * t82803 * t1950 + 0.80418998823691070229e-1_f64 * t82806 - 0.54831135561607547884e-2_f64 * t82809 + 3.0_f64 * t11051 * t6811 + t11046 * t82730 * t11048 - 3.0_f64 * t3200 * t23704 * t4684 + 6.0_f64 * t3186 * t23704 * t4673 + 0.54831135561607547883e-2_f64 * t82823 + 6.0_f64 * t11034 * t23701 + 0.82246703342411321826e-2_f64 * t82828 + 0.14621636149762012769e-1_f64 * t82830 + 6.0_f64 * t3180 * t23654;
    t82834
}
