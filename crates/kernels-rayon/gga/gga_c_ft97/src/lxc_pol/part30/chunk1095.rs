//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1095/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1095(t24980: f64, t2862: f64, t28741: f64, t33868: f64, t28735: f64, t28736: f64, t840: f64, t143042: f64, t143112: f64, t28496: f64, t33820: f64, t143040: f64, t143041: f64, t28511: f64) -> (f64, f64, f64, f64) {
    let t152690 = t24980 * t2862 * t33868 * t28741;
    let t152694 = t28735 * t840 * t33868 * t28736;
    let t152698 = t33820 * t143112 * t143042 * t28496;
    let t152702 = t143040 * t143041 * t143042 * t28511;
    (t152690, t152694, t152698, t152702)
}
