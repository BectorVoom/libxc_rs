//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 926/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk926(t12411: f64, t135: f64, t5820: f64, t12374: f64, t6608: f64, t94765: f64, t23809: f64, t3347: f64, t23724: f64, t6604: f64, t1391: f64, t2101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104819 = t12411 * t135 * t5820;
    let t104860 = t12374 * t5820;
    let t105080 = t94765 * t6608;
    let t105135 = t3347 * t23809;
    let t105260 = t12411 * t23809;
    let t105279 = t23724 * t6604;
    let t106296 = t2101 * t1391;
    (t104819, t104860, t105080, t105135, t105260, t105279, t106296)
}
