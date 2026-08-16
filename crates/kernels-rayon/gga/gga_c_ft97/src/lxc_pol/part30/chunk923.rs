//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 923/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk923(t1495: f64, t799: f64, t24898: f64, t56110: f64, t10697: f64, t1508: f64, t2770: f64, t7091: f64, t848: f64, t29055: f64, t10261: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112920 = t799 * t1495;
    let t112952 = t56110 * t24898;
    let t112975 = t799 * t10697;
    let t112987 = t2770 * t1508;
    let t113656 = t848 * t7091;
    let t114271 = t56110 * t29055;
    let t114531 = t10261 * t871;
    (t112920, t112952, t112975, t112987, t113656, t114271, t114531)
}
