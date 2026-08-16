//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 915/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk915(t24898: f64, t56456: f64, t10696: f64, t1495: f64, t263: f64, t27742: f64, t22511: f64, t27519: f64, t3789: f64, t3758: f64, t695: f64, t200: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99672 = t56456 * t24898;
    let t99918 = t1495 * t10696;
    let t107910 = t27742 * t263;
    let t108446 = t27519 * t22511;
    let t108447 = t3789 * t108446;
    let t108517 = t3758 * t695;
    let t108530 = t200 * t668;
    (t99672, t99918, t107910, t108446, t108447, t108517, t108530)
}
