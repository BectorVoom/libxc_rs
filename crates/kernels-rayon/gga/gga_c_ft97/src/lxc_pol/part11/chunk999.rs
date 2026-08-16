//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 999/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk999(t8392: f64, t9118: f64, t1570: f64, t2178: f64, t1559: f64, t2180: f64, t1557: f64, t604: f64, t7800: f64, t609: f64, t7765: f64, t2225: f64, t38953: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40757 = t8392 * t9118;
    let t40759 = t2178 * t1570;
    let t40760 = t1559 * t2180;
    let t40766 = t2178 * t1557;
    let t40771 = t604 * t7800;
    let t40772 = t7765 * t609;
    let t40777 = t38953 * t2225;
    (t40757, t40759, t40760, t40766, t40771, t40772, t40777)
}
