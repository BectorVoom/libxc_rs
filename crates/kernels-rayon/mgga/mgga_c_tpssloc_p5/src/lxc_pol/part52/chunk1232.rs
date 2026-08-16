//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1232/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1232(t7754: f64, t8690: f64, t113: f64, t33096: f64, t33098: f64, t33100: f64, t33101: f64, t33127: f64, t33131: f64, t33134: f64, t33139: f64, t33158: f64, t33162: f64, t33747: f64, t33748: f64, t33756: f64) -> f64 {
    let t33758 = t8690 * t7754;
    let t33759 = -t113 * t33756 - t33096 - t33098 - t33100 - 2.0_f64 * t33101 + t33127 + t33131 + t33134 - t33139 - t33158 - t33162 + t33747 + 3.0_f64 * t33748 + t33758;
    t33759
}
