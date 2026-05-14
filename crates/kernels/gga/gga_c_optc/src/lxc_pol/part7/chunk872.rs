//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 872/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk872<F: Float>(t415: F, t8868: F, t8639: F, t8589: F, t8591: F, t8593: F, t8603: F, t8606: F, t8609: F, t8622: F, t8625: F, t8657: F, t8660: F, t389: F, t1067: F, t1095: F, t2937: F, t2974: F, t402: F, t8560: F, t8564: F, t8571: F, t8574: F, t8576: F, t8579: F, t8585: F, t8682: F, t8691: F, t8806: F, t8809: F, t8843: F, t8848: F, t8851: F, t8854: F) -> (F, F, F, F, F) {
    let t8869 = t8868 * t415;
    let t8871 = 0.53272592592592592592e-1 * t8639;
    let t8882 = -t8871 - 0.2283111111111111111e-1 * t8589 + 0.11415555555555555555e-1 * t8593 - 0.34246666666666666665e-1 * t8603 + 0.17123333333333333333e-1 * t8591 - 0.19025925925925925925e-1 * t8622 + 0.68493333333333333331e-1 * t8606 - 0.34246666666666666665e-1 * t8657 - 0.10274e0 * t8609 + 0.10274e0 * t8660 - 0.17123333333333333333e-1 * t8625;
    let t8885 = 0.55403703703703703703e-1 * t8639;
    let t8896 = -t8885 - 0.23744444444444444444e-1 * t8589 + 0.11872222222222222222e-1 * t8593 - 0.35616666666666666666e-1 * t8603 + 0.17808333333333333333e-1 * t8591 - 0.19787037037037037037e-1 * t8622 + 0.71233333333333333332e-1 * t8606 - 0.35616666666666666666e-1 * t8657 - 0.10685e0 * t8609 + 0.10685e0 * t8660 - 0.17808333333333333333e-1 * t8625;
    let t8898 = 0.62182e-1 * t8896 * t389;
    let t8899 = -t8571 - t8576 - t8579 + t8585 - t8682 - t8691 - 6.0 * t8806 * t2937 + 6.0 * t2974 * t8809 + 1.0 * t1067 * t8843 + 0.20691336878655965246e4 * t8848 * t8851 + 0.17544670192365612213e1 * t8854 * t1095 + t8560 - t8564 - t8574 - 0.19751789702565206229e-1 * t8869 - 0.3109e-1 * t8882 * t402 + t8898;
    (t8869, t8882, t8896, t8898, t8899)
}
