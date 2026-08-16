//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 873/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk873(t13602: f64, t701: f64, t420: f64, t9651: f64, t13315: f64, t2248: f64, t2440: f64, t13320: f64, t13296: f64, t2320: f64, t703: f64, t13301: f64) -> (f64, f64, f64, f64, f64) {
    let t13603 = t701 * t13602;
    let t13605 = t420 * t9651;
    let t13606 = t13605 * t13315;
    let t13607 = t701 * t13606;
    let t13609 = t2248 * t2440;
    let t13610 = t13609 * t13320;
    let t13611 = t701 * t13610;
    let t13613 = t2320 * t13296;
    let t13614 = t701 * t13613;
    let t13616 = t2248 * t703;
    let t13617 = t13616 * t13301;
    (t13603, t13607, t13611, t13614, t13617)
}
