//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 591/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk591(t1113: f64, t703: f64, t684: f64, t22511: f64, t6813: f64, t3789: f64) -> (f64, f64, f64) {
    let t27652 = t703 * t1113;
    let t27653 = t27652 * t684;
    let t27657 = t6813 * t22511;
    let t27658 = t3789 * t27657;
    (t27653, t27657, t27658)
}
