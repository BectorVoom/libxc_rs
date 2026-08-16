//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 867/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk867(t173: f64, t419: f64, t8089: f64, t422: f64, t7800: f64, t37357: f64, t420: f64, t1742: f64, t37362: f64, t1744: f64, t8130: f64, t1725: f64, t8126: f64) -> (f64, f64, f64, f64, f64) {
    let t37763 = t419 * t173 * t8089;
    let t37765 = t422 * t7800;
    let t37768 = t419 * t420 * t37765 * t37357;
    let t37772 = t419 * t420 * t1742 * t37362;
    let t37774 = t8130 * t1744;
    let t37776 = t1725 * t8126;
    (t37763, t37768, t37772, t37774, t37776)
}
