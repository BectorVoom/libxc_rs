//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1462/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1462(t10544: f64, t10530: f64, t10538: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t894: f64) -> (f64, f64, f64) {
    let t10577 = 28.0_f64 / 27.0_f64 * t10544;
    let t10588 = -t10577 - 4.0_f64 / 9.0_f64 * t10556 + 2.0_f64 / 9.0_f64 * t10558 - 2.0_f64 / 3.0_f64 * t10560 + t10562 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t10566 + 4.0_f64 / 3.0_f64 * t10569 - 2.0_f64 / 3.0_f64 * t10530 - 2.0_f64 * t10572 + 2.0_f64 * t10538 - t10575 / 3.0_f64;
    let t10589 = t894 * t10588;
    (t10577, t10588, t10589)
}
