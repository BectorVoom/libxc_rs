//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 705/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk705(t2417: f64, t689: f64, t2394: f64, t2418: f64, t709: f64, t680: f64, t2455: f64, t2379: f64, t9538: f64, t209: f64, t2247: f64, t228: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9613 = t689 * t2417;
    let t9614 = t2394 * t9613;
    let t9617 = t2418 * t709;
    let t9618 = t680 * t9617;
    let t9621 = t689 * t2455;
    let t9622 = t680 * t9621;
    let t9625 = t2379 * t9538;
    let t9631 = t2379 * t9613;
    let t9634 = t209 * t2247;
    let t9636 = t228 * t9634 * t231;
    (t9614, t9617, t9618, t9622, t9625, t9631, t9636)
}
