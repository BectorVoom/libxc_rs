//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 870/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk870(t13567: f64, t26: f64, t2999: f64, t13538: f64, t13541: f64, t13543: f64, t13544: f64, t13547: f64, t13550: f64, t13553: f64, t13556: f64, t13559: f64, t13562: f64, t13565: f64, t9557: f64, t9558: f64, t9560: f64, t9562: f64, t9564: f64) -> (f64, f64) {
    let t13569 = t26 * t2999 * t13567;
    let t13571 = -t9557 - 8.0_f64 / 27.0_f64 * t9558 + 2.0_f64 / 27.0_f64 * t9560 - 2.0_f64 / 9.0_f64 * t9562 + t9564 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t13538 + t13541 - t13543 - 22.0_f64 / 9.0_f64 * t13544 - 10.0_f64 / 27.0_f64 * t13547 + 4.0_f64 / 3.0_f64 * t13550 + 8.0_f64 / 9.0_f64 * t13553 - 2.0_f64 / 9.0_f64 * t13556 - 2.0_f64 * t13559 - 8.0_f64 / 3.0_f64 * t13562 + 2.0_f64 / 3.0_f64 * t13565 + 2.0_f64 / 3.0_f64 * t13569;
    (t13569, t13571)
}
