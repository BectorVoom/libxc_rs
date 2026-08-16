//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 881/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk881(t2: f64, t33300: f64, t626: f64, t703: f64, t240: f64, t9577: f64, t342: f64, t657: f64, t8639: f64, t9570: f64, t762: f64, t9895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42218 = t33300 * t2;
    let t42262 = t626 * t703;
    let t42279 = t240 * t9577;
    let t42293 = 5.0_f64 / 54.0_f64 * t342 * t8639 * t657;
    let t42307 = t240 * t9570;
    let t42334 = t9895 * t762;
    (t42218, t42262, t42279, t42293, t42307, t42334)
}
