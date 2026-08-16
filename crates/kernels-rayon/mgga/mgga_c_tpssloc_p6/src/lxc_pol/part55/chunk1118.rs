//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1118/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1118(t5: f64, t7973: f64, t8307: f64, t8513: f64, t32579: f64, t32583: f64, t32590: f64, t33103: f64, t33107: f64, t33111: f64, t33119: f64, t8663: f64, t8856: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t34221 = t8307 * t7973;
    let t34222 = t8513 * t34221;
    let t34228 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t33103 * t8856 - 5.0_f64 / 24.0_f64 * t32579 * t33107 - 5.0_f64 / 36.0_f64 * t32583 * t33111 + 5.0_f64 / 72.0_f64 * t8663 * t34222 + 5.0_f64 / 72.0_f64 * t32590 * t33119);
    (t34221, t34222, t34228)
}
