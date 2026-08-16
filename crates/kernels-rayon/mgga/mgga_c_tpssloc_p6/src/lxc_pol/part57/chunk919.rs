//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 919/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk919(t115390: f64, t22724: f64, t31623: f64, t22716: f64, t8631: f64, t113981: f64, t114025: f64, t114027: f64, t114038: f64, t3787: f64, t8617: f64, t31594: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t115391 = 0.82246703342411321824e-2_f64 * t115390;
    let t115432 = t22724 * t31623;
    let t115433 = 0.26044789391763585244e-1_f64 * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = 0.63969658155208805863e-1_f64 * t115434;
    let t115447 = 0.13457585364713463618e-3_f64 * t113981;
    let t115461 = 0.42167100809435519335e-2_f64 * t114025;
    let t115462 = 0.90434973650874475512e-1_f64 * t114027;
    let t115465 = 119.0_f64 / 3456.0_f64 * t114038;
    let t115494 = t3787 * t8617;
    let t115539 = t22724 * t31594;
    (t115391, t115433, t115435, t115447, t115461, t115462, t115465, t115494, t115539)
}
