//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 519/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk519(t210: f64, t214: f64, t6330: f64, t6347: f64, t1315: f64, t3725: f64, t3731: f64, t3733: f64, t3751: f64, t5192: f64, t5203: f64) -> f64 {
    let t6353 = t210 * t214 * t6330;
    let t6358 = t210 * t214 * t6347;
    let t6361 = t3725 + 0.77777777777777777775e-2_f64 * t5192 + t3731 + 0.49999999999999999998e-2_f64 * t3733 * t6353 + 0.16666666666666666666e-2_f64 * t5203 - 0.16666666666666666666e-2_f64 * t1315 * t6358 - t3751;
    t6361
}
