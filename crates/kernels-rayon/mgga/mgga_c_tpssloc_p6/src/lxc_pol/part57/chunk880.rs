//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 880/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk880(t1912: f64, t25168: f64, t259: f64, t26713: f64, t30655: f64, t30662: f64, t31350: f64, t32865: f64, t32869: f64, t33405: f64, t33410: f64, t33412: f64, t33414: f64, t6627: f64, t7842: f64) -> f64 {
    let t33416 = -t6627 * t7842 - t26713 * t1912 - 6.0_f64 * t25168 * t33405 - 0.82246703342411321825e-2_f64 * t33410 - t30655 + t32865 - t32869 + t30662 - t31350 + t33412 * t259 + t33414 * t259;
    t33416
}
