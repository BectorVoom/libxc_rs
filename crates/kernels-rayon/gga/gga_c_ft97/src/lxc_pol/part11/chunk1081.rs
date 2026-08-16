//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1081/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1081(t10020: f64, t1882: f64, t9840: f64, t10131: f64, t10002: f64, t10024: f64, t10029: f64, t10034: f64, t2459: f64, t2469: f64, t2568: f64, t2569: f64, t2574: f64, t2594: f64, t265: f64, t41753: f64, t41794: f64, t42455: f64, t42469: f64, t446: f64, t729: f64, t773: f64, t9572: f64, t9578: f64) -> f64 {
    let t42474 = t1882 * t10020;
    let t42476 = t1882 * t9840;
    let t42482 = t1882 * t10131;
    let t42488 = -4.0_f64 * t446 * t729 * t2568 * t2569 * t2459 + 16.0_f64 / 9.0_f64 * t42455 - 8.0_f64 * t446 * t2574 * t2469 * t10029 - 8.0_f64 * t446 * t729 * t10002 * t10034 - 40.0_f64 / 81.0_f64 * t446 * t10024 * t773 * t9572 - 80.0_f64 / 243.0_f64 * t446 * t42469 * t265 * t41753 - 4.0_f64 / 3.0_f64 * t42474 - 8.0_f64 / 3.0_f64 * t42476 + 16.0_f64 / 9.0_f64 * t446 * t2594 * t773 * t9578 + 4.0_f64 / 27.0_f64 * t42482 - t446 * t729 * t265 * t41794 / 3.0_f64;
    t42488
}
