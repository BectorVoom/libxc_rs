//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 869/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk869(t37357: f64, t37789: f64, t419: f64, t420: f64, t1725: f64, t8098: f64, t1743: f64, t626: f64, t8115: f64, t8122: f64, t1737: f64, t37362: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37792 = t419 * t420 * t37789 * t37357;
    let t37795 = t1725 * t8098;
    let t37798 = t419 * t626 * t1743;
    let t37800 = t1725 * t8115;
    let t37802 = t1725 * t8122;
    let t37806 = t419 * t420 * t1737 * t37362;
    (t37792, t37795, t37798, t37800, t37802, t37806)
}
