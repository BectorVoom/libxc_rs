//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2692/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2692(t19767: f64, t40409: f64, t19771: f64, t3726: f64, t12199: f64, t19775: f64, t40387: f64, t40401: f64, t40402: f64, t40404: f64, t40407: f64, t40410: f64, t40422: f64, t40425: f64, t54663: f64, t54667: f64, t54671: f64) -> f64 {
    let t56535 = t40409 * t19767;
    let t56537 = t3726 * t19771;
    let t56539 = t12199 * t19775;
    let t56542 = 0.38888888888888888889e-1_f64 * t40387 - t40401 + 0.11234567901234567901e0_f64 * t40402 - 0.12962962962962962963e-1_f64 * t40404 + 0.6574074074074074074e-1_f64 * t40407 + 0.15833333333333333333e-1_f64 * t40410 + t40422 - 0.52777777777777777776e-2_f64 * t40425 - 0.39999999999999999998e-1_f64 * t54663 + 0.66666666666666666664e-2_f64 * t54667 + 0.15833333333333333333e-1_f64 * t56535 + 0.77777777777777777774e-2_f64 * t56537 - 0.52777777777777777776e-2_f64 * t56539 + 0.93333333333333333328e-1_f64 * t54671;
    t56542
}
