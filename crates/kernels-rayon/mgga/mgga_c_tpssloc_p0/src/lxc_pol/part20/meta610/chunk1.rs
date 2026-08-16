//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2197/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2197(t11154: f64, t11784: f64, t1227: f64, t248: f64, t11814: f64, t3572: f64, t11825: f64, t3523: f64, t11820: f64, t3536: f64, t11778: f64, t121: f64) -> (f64, f64, f64, f64, f64) {
    let t45260 = t1227 * t248 * t11784 * t11154;
    let t45262 = t11814 * t3572;
    let t45264 = t11825 * t3523;
    let t45266 = t3536 * t11820;
    let t45268 = t121 * t11778;
    (t45260, t45262, t45264, t45266, t45268)
}
