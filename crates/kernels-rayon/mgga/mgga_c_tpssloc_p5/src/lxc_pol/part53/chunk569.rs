//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 569/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk569(t210: f64, t214: f64, t5187: f64, t1315: f64, t3725: f64, t3727: f64, t3731: f64, t3742: f64, t3751: f64, t5192: f64, t5195: f64, t5198: f64, t5203: f64) -> f64 {
    let t5206 = t210 * t214 * t5187;
    let t5210 = t3725 + 0.38888888888888888888e-2_f64 * t3727 + t3731 + 0.38888888888888888887e-2_f64 * t5192 + 0.49999999999999999998e-2_f64 * t5195 * t5198 + 0.8333333333333333333e-3_f64 * t5203 - 0.16666666666666666666e-2_f64 * t1315 * t5206 + 0.83333333333333333332e-3_f64 * t3742 - t3751;
    t5210
}
