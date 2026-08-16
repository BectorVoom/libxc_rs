//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 807/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk807(t1045: f64, t582: f64, t2213: f64, t1017: f64, t2230: f64, t574: f64, t1060: f64, t2075: f64, t12561: f64, t167: f64, t3408: f64, t616: f64) -> (f64, f64, f64, f64, f64) {
    let t12680 = t582 * t1045;
    let t12681 = t12680 * t2213;
    let t12685 = t574 * t2230 * t1017;
    let t12689 = t574 * t1060 * t2075;
    let t12696 = t574 * t167 * t12561;
    let t12700 = t574 * t616 * t3408;
    (t12681, t12685, t12689, t12696, t12700)
}
