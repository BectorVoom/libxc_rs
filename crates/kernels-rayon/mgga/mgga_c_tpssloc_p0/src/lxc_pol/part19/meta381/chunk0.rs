//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1423/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1423(t43776: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64) -> f64 {
    let t44027 = 0.13388493827160493828e1_f64 * t43776;
    let t44036 = 0.49293999999999999999e0_f64 * t43759 - 0.85199506172839506175e-1_f64 * t43766 + 0.21908444444444444444e0_f64 * t43768 - 0.13145066666666666666e1_f64 * t43770 + 0.21908444444444444444e0_f64 * t43773 + t44027 + 0.3071625e0_f64 * t43833 + 0.43816888888888888888e0_f64 * t43835 - 0.13145066666666666666e1_f64 * t43837 - 0.21908444444444444444e0_f64 * t43839 + 0.43816888888888888889e0_f64 * t43842 - 0.98587999999999999998e0_f64 * t43845 + 0.197176e1_f64 * t43848 + 0.82156666666666666667e-1_f64 * t43851;
    t44036
}
