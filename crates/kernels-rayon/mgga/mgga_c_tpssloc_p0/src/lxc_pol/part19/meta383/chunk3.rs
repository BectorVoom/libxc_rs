//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1434/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1434(t43819: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64) -> f64 {
    let t44275 = 0.5356037037037037037e1_f64 * t43819;
    let t44289 = t44275 + 0.13772666666666666666e1_f64 * t43780 + 0.27545333333333333333e1_f64 * t43782 + 0.27545333333333333332e1_f64 * t43784 - 0.41318e1_f64 * t43786 - 0.68863333333333333332e0_f64 * t43788 + 0.68863333333333333334e1_f64 * t43794 - 0.123954e2_f64 * t43798 + 0.123954e2_f64 * t43802 + 0.516475e0_f64 * t43806 - 0.15302962962962962963e1_f64 * t43811 - 0.21424148148148148148e1_f64 * t43816 - 0.103295e1_f64 * t43823 + 0.309885e1_f64 * t43828;
    t44289
}
