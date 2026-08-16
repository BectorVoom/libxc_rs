//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 475/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk475(t2739: f64, t799: f64, t27: f64, t89: f64, t2653: f64, t2656: f64, t2659: f64, t2663: f64, t2668: f64, t2673: f64, t2677: f64, t2685: f64) -> (f64, f64, f64) {
    let t2740 = t799 * t2739;
    let t2742 = t89 * t27 * t2740;
    let t2744 = t2653 + t2656 + t2659 - t2663 / 27.0_f64 + t2668 / 9.0_f64 + t2673 / 9.0_f64 - t2677 / 18.0_f64 + t2685 / 3.0_f64 - t2742 / 6.0_f64;
    (t2740, t2742, t2744)
}
