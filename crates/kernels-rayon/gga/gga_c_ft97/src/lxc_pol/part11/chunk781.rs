//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 781/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk781(t10575: f64, t2681: f64, t2739: f64, t295: f64, t9567: f64, t9954: f64, t2783: f64, t458: f64, t8282: f64, t849: f64, t10556: f64, t10559: f64, t10560: f64, t10563: f64, t10566: f64, t10568: f64, t10572: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64) {
    let t10577 = t2681 * t10575 * t2739;
    let t10580 = t9567 * t295;
    let t10581 = t10580 * t9954;
    let t10584 = t458 * t2783;
    let t10586 = t8282 * t849;
    let t10588 = -t462 * t10556 / 3.0_f64 + t10559 + 4.0_f64 / 3.0_f64 * t462 * t10560 - 2.0_f64 / 3.0_f64 * t462 * t10563 + t462 * t10566 + t462 * t10568 - 6.0_f64 * t92 * t10572 + 6.0_f64 * t462 * t10577 - 10.0_f64 / 27.0_f64 * t462 * t10581 - 2.0_f64 * t10584 - 4.0_f64 / 9.0_f64 * t10586;
    (t10577, t10580, t10581, t10588)
}
