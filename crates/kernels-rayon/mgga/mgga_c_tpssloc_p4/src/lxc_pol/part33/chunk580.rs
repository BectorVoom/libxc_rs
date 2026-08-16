//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 580/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk580(t210: f64, t214: f64, t5527: f64, t5544: f64, t2562: f64, t2569: f64, t2571: f64, t2590: f64, t4124: f64, t4135: f64, t787: f64) -> (f64, f64, f64) {
    let t5550 = t210 * t214 * t5527;
    let t5555 = t210 * t214 * t5544;
    let t5558 = t2562 + 0.77777777777777777775e-2_f64 * t4124 + t2569 + 0.49999999999999999998e-2_f64 * t2571 * t5550 + 0.16666666666666666666e-2_f64 * t4135 - 0.16666666666666666666e-2_f64 * t787 * t5555 - t2590;
    (t5550, t5555, t5558)
}
