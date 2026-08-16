//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 628/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk628(t179: f64, t7800: f64, t2258: f64, t7765: f64, t3613: f64, t7807: f64, t1651: f64, t643: f64, t2266: f64, t2294: f64, t379: f64, t2252: f64, t41: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8660 = t179 * t7800;
    let t8662 = t2258 * t8660 * t7765;
    let t8665 = t3613 * t7807;
    let t8668 = t1651 * t643;
    let t8669 = t2266 * t8668;
    let t8671 = t379 * t2294;
    let t8672 = t2266 * t8671;
    let t8675 = t41 * t2252 * t70;
    (t8660, t8662, t8665, t8669, t8671, t8672, t8675)
}
