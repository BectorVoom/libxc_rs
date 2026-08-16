//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 930/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk930(t8639: f64, t8589: f64, t8591: f64, t8593: f64, t8603: f64, t8606: f64, t8609: f64, t8622: f64, t8625: f64, t8657: f64, t8660: f64, t389: f64) -> (f64, f64, f64) {
    let t8871 = 0.53272592592592592592e-1_f64 * t8639;
    let t8882 = -t8871 - 0.2283111111111111111e-1_f64 * t8589 + 0.11415555555555555555e-1_f64 * t8593 - 0.34246666666666666665e-1_f64 * t8603 + 0.17123333333333333333e-1_f64 * t8591 - 0.19025925925925925925e-1_f64 * t8622 + 0.68493333333333333331e-1_f64 * t8606 - 0.34246666666666666665e-1_f64 * t8657 - 0.10274e0_f64 * t8609 + 0.10274e0_f64 * t8660 - 0.17123333333333333333e-1_f64 * t8625;
    let t8885 = 0.55403703703703703703e-1_f64 * t8639;
    let t8896 = -t8885 - 0.23744444444444444444e-1_f64 * t8589 + 0.11872222222222222222e-1_f64 * t8593 - 0.35616666666666666666e-1_f64 * t8603 + 0.17808333333333333333e-1_f64 * t8591 - 0.19787037037037037037e-1_f64 * t8622 + 0.71233333333333333332e-1_f64 * t8606 - 0.35616666666666666666e-1_f64 * t8657 - 0.10685e0_f64 * t8609 + 0.10685e0_f64 * t8660 - 0.17808333333333333333e-1_f64 * t8625;
    let t8898 = 0.62182e-1_f64 * t8896 * t389;
    (t8882, t8896, t8898)
}
