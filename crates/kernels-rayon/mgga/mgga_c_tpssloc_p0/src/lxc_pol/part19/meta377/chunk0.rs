//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1409/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409(t1113: f64, t136: f64, t43800: f64, t43804: f64, t43759: f64, t43766: f64, t43768: f64, t43770: f64, t43773: f64, t43777: f64, t43833: f64, t43835: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64) -> (f64, f64, f64) {
    let t43848 = t136 * t1113 * t43800;
    let t43851 = t136 * t1113 * t43804;
    let t43853 = 0.49671e0_f64 * t43759 - 0.8585111111111111111e-1_f64 * t43766 + 0.22076e0_f64 * t43768 - 0.132456e1_f64 * t43770 + 0.22076e0_f64 * t43773 + t43777 + 0.16504875e0_f64 * t43833 + 0.44152e0_f64 * t43835 - 0.132456e1_f64 * t43837 - 0.22076e0_f64 * t43839 + 0.44152e0_f64 * t43842 - 0.99342e0_f64 * t43845 + 0.198684e1_f64 * t43848 + 0.82785e-1_f64 * t43851;
    (t43848, t43851, t43853)
}
