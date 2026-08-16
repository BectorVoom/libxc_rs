//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 907/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk907(t4591: f64, t8232: f64, t38953: f64, t4613: f64, t4608: f64, t4561: f64, t4557: f64, t1851: f64, t4545: f64, t2252: f64, t342: f64, t4410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60398 = t8232 * t4591;
    let t60433 = t38953 * t4613;
    let t60756 = t38953 * t4608;
    let t60919 = t8232 * t4561;
    let t60984 = t8232 * t4557;
    let t61025 = t4545 * t1851;
    let t61180 = t342 * t2252 * t4410;
    (t60398, t60433, t60756, t60919, t60984, t61025, t61180)
}
