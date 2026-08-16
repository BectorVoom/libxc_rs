//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1432/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1432(t43776: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64) -> f64 {
    let t44249 = 0.16979925925925925926e1_f64 * t43776;
    let t44258 = 0.62517e0_f64 * t43759 - 0.10805407407407407407e0_f64 * t43766 + 0.27785333333333333333e0_f64 * t43768 - 0.166712e1_f64 * t43770 + 0.27785333333333333334e0_f64 * t43773 + t44249 + 0.6311625e0_f64 * t43833 + 0.55570666666666666668e0_f64 * t43835 - 0.166712e1_f64 * t43837 - 0.27785333333333333333e0_f64 * t43839 + 0.55570666666666666666e0_f64 * t43842 - 0.125034e1_f64 * t43845 + 0.250068e1_f64 * t43848 + 0.104195e0_f64 * t43851;
    t44258
}
