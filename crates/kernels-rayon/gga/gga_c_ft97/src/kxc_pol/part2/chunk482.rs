//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 482/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk482(t2789: f64, t845: f64, t91: f64, t2652: f64, t2655: f64, t2658: f64, t2663: f64, t2668: f64, t2673: f64, t2677: f64, t2685: f64, t2742: f64, t2758: f64) -> (f64, f64, f64) {
    let t2791 = t91 * t845 * t2789;
    let t2793 = 4.0_f64 / 9.0_f64 * t2652;
    let t2801 = -t2758 / 4.0_f64 + t2791 / 2.0_f64 + t2793 + 2.0_f64 / 9.0_f64 * t2655 + 2.0_f64 / 3.0_f64 * t2658 - 2.0_f64 / 9.0_f64 * t2663 + 2.0_f64 / 3.0_f64 * t2668 + 2.0_f64 / 3.0_f64 * t2673 - t2677 / 3.0_f64 + 2.0_f64 * t2685 - t2742;
    (t2791, t2793, t2801)
}
