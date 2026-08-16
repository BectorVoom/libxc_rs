//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1476/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1476(t210: f64, t214: f64, t6330: f64, t6347: f64, t1315: f64, t3725: f64, t3731: f64, t3733: f64, t3751: f64, t5192: f64, t5203: f64, t562: f64) -> (f64, f64, f64, f64) {
    let t6353 = t210 * t214 * t6330;
    let t6358 = t210 * t214 * t6347;
    let t6361 = t3725 + 0.77777777777777777775e-2_f64 * t5192 + t3731 + 0.49999999999999999998e-2_f64 * t3733 * t6353 + 0.16666666666666666666e-2_f64 * t5203 - 0.16666666666666666666e-2_f64 * t1315 * t6358 - t3751;
    let t6362 = t6361 * t562;
    (t6353, t6358, t6361, t6362)
}
