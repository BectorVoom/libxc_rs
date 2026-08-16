//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 511/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk511(t222: f64, t226: f64, t2248: f64, t703: f64, t3951: f64, t761: f64, t1160: f64, t737: f64, t2372: f64, t255: f64) -> (f64, f64, f64, f64, f64) {
    let t13580 = t222 * t226;
    let t13616 = t2248 * t703;
    let t13830 = t3951 * t761;
    let t13839 = t737 * t1160;
    let t13885 = t2372 * t255;
    (t13580, t13616, t13830, t13839, t13885)
}
