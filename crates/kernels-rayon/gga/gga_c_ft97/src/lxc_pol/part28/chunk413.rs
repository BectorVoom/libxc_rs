//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 413/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk413(t5570: f64, t6441: f64, t72: f64, t938: f64, t5579: f64, t2258: f64, t925: f64) -> (f64, f64, f64, f64) {
    let t6442 = t5570 * t6441;
    let t6445 = t72 * t938;
    let t6446 = t5579 * t6445;
    let t6449 = t2258 * t925;
    (t6442, t6445, t6446, t6449)
}
