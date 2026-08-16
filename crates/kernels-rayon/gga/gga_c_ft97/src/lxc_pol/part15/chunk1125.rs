//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1125/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1125(t2320: f64, t701: f64, t88606: f64, t21197: f64, t3799: f64, t17748: f64, t4635: f64) -> (f64, f64, f64) {
    let t88608 = t701 * t2320 * t88606;
    let t88610 = t3799 * t21197;
    let t88612 = t17748 * t4635;
    (t88608, t88610, t88612)
}
