//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 862/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk862(t625: f64, t8307: f64, t8513: f64, t8663: f64, t111: f64, t8828: f64) -> (f64, f64, f64, f64) {
    let t32343 = t8307 * t625;
    let t32344 = t8513 * t32343;
    let t32346 = 5.0_f64 / 27.0_f64 * t8663 * t32344;
    let t32350 = t8828 * t111;
    (t32343, t32344, t32346, t32350)
}
