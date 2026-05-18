//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 930/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk930<F: Float>(t8639: F, t8589: F, t8591: F, t8593: F, t8603: F, t8606: F, t8609: F, t8622: F, t8625: F, t8657: F, t8660: F, t389: F) -> (F, F, F) {
    let t8871 = F::new(0.53272592592592592592e-1) * t8639;
    let t8882 = -t8871 - F::new(0.2283111111111111111e-1) * t8589 + F::new(0.11415555555555555555e-1) * t8593 - F::new(0.34246666666666666665e-1) * t8603 + F::new(0.17123333333333333333e-1) * t8591 - F::new(0.19025925925925925925e-1) * t8622 + F::new(0.68493333333333333331e-1) * t8606 - F::new(0.34246666666666666665e-1) * t8657 - F::new(0.10274e0) * t8609 + F::new(0.10274e0) * t8660 - F::new(0.17123333333333333333e-1) * t8625;
    let t8885 = F::new(0.55403703703703703703e-1) * t8639;
    let t8896 = -t8885 - F::new(0.23744444444444444444e-1) * t8589 + F::new(0.11872222222222222222e-1) * t8593 - F::new(0.35616666666666666666e-1) * t8603 + F::new(0.17808333333333333333e-1) * t8591 - F::new(0.19787037037037037037e-1) * t8622 + F::new(0.71233333333333333332e-1) * t8606 - F::new(0.35616666666666666666e-1) * t8657 - F::new(0.10685e0) * t8609 + F::new(0.10685e0) * t8660 - F::new(0.17808333333333333333e-1) * t8625;
    let t8898 = F::new(0.62182e-1) * t8896 * t389;
    (t8882, t8896, t8898)
}
