//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 605/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk605(t25759: f64, t379: f64, t22572: f64, t5569: f64, t6441: f64, t22798: f64, t6426: f64, t22797: f64, t11247: f64, t384: f64) -> (f64, f64, f64, f64, f64) {
    let t25760 = t25759 * t379;
    let t25768 = t5569 * t22572 * t6441;
    let t25770 = t6426 * t22798;
    let t25771 = t22797 * t25770;
    let t25774 = t11247 * t384;
    (t25760, t25768, t25770, t25771, t25774)
}
