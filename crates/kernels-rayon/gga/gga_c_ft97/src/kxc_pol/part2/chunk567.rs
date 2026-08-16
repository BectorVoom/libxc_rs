//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 567/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk567(t3740: f64, t92: f64, t3700: f64, t683: f64, t18: f64, t668: f64) -> (f64, f64, f64, f64) {
    let t3741 = t92 * t3740;
    let t3743 = t683 * t3700;
    let t3744 = t92 * t3743;
    let t3746 = t668 * t18;
    (t3741, t3743, t3744, t3746)
}
