//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 374/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk374(t488: f64, t5743: f64, t83: f64, t1901: f64, t28: f64, t446: f64, t5629: f64, t5632: f64, t5637: f64, t5641: f64, t5646: f64, t5650: f64, t5655: f64, t5657: f64, t5661: f64, t5706: f64, t5712: f64, t5716: f64, t5719: f64, t5724: f64, t5728: f64, t5733: f64, t89: f64) -> (f64, f64) {
    let t5744 = t488 * t5743;
    let t5745 = t83 * t5744;
    let t5748 = t5629 + t1901 * t5632 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t5637 - t446 * t5641 / 3.0_f64 + t446 * t5646 / 3.0_f64 - t446 * t5650 / 3.0_f64 - t5655 - t446 * t5657 / 9.0_f64 - t446 * t5661 / 3.0_f64 + t89 * t28 * t5706 / 3.0_f64 - t446 * t5712 / 3.0_f64 + t5716 + t1901 * t5719 / 9.0_f64 + t446 * t5724 / 3.0_f64 - t446 * t5728 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t5733 - t446 * t5745 / 3.0_f64;
    (t5745, t5748)
}
