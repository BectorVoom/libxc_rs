//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1730/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1730(t1983: f64, t22585: f64, t1266: f64, t1393: f64, t1869: f64, t1976: f64, t1980: f64, t22460: f64, t22461: f64, t22467: f64, t22482: f64, t22483: f64, t22559: f64, t22563: f64, t22577: f64, t22580: f64, t22583: f64, t2314: f64, t2320: f64, t2323: f64, t3652: f64, t3929: f64, t510: f64, t650: f64, t6515: f64, t6517: f64, t652: f64, t6539: f64, t672: f64, t6862: f64, t6872: f64) -> f64 {
    let t22587 = 3.0_f64 * t1983 * t22585;
    let t22588 = -2.0_f64 * t1266 * t6515 + 2.0_f64 * t1393 * t6872 - t1869 * t3652 - 2.0_f64 * t1976 * t2320 + t1980 * t3929 - 4.0_f64 * t22461 * t672 - 2.0_f64 * t22483 * t652 - t22559 * t510 - 4.0_f64 * t2314 * t6539 - 4.0_f64 * t2323 * t6517 - 2.0_f64 * t650 * t6862 - t22460 - t22467 - t22482 - t22563 - t22577 - t22580 - t22583 + t22587;
    t22588
}
