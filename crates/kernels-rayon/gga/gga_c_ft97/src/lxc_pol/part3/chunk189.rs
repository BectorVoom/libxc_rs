//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 189/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk189(t149: f64, t165: f64, t564: f64, t610: f64, t614: f64, t616: f64, t184: f64, t169: f64, t5: f64, t13: f64, t171: f64) -> (f64, f64, f64, f64) {
    let t619 = -t149 * t614 - t165 * t564 - 2.0_f64 * t610 + 2.0_f64 * t616;
    let t620 = t619 * t184;
    let t623 = t5 * t169;
    let t625 = 1.0_f64 / t171 / t13;
    (t619, t620, t623, t625)
}
