//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 800/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk800(t12587: f64, t28: f64, t89: f64, t2223: f64, t2983: f64, t9049: f64, t446: f64, t12346: f64, t12353: f64, t12357: f64, t12359: f64, t12362: f64, t12366: f64, t12564: f64, t12568: f64, t12571: f64, t12574: f64, t12577: f64, t12580: f64, t12584: f64, t8799: f64, t8802: f64, t9059: f64, t9062: f64, t9072: f64) -> (f64, f64, f64, f64) {
    let t12589 = t89 * t28 * t12587;
    let t12590 = t2983 * t2223;
    let t12591 = t9049 * t12590;
    let t12592 = t446 * t12591;
    let t12595 = -t12346 + t8799 / 54.0_f64 + t8802 / 81.0_f64 - t9059 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t12353 - t12357 + 11.0_f64 / 27.0_f64 * t12359 - 2.0_f64 / 81.0_f64 * t12362 - t9072 + t12366 - t12564 / 6.0_f64 - t12568 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t12571 + t12574 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t12577 - t12580 / 27.0_f64 + t12584 / 3.0_f64 - t12589 + 2.0_f64 / 27.0_f64 * t12592 - t9062 / 27.0_f64;
    (t12589, t12590, t12592, t12595)
}
