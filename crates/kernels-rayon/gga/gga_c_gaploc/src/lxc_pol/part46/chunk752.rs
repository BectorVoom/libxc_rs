//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 752/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk752(t29975: f64, t6508: f64, t2293: f64, t874: f64, t172: f64, t20368: f64, t2366: f64, t29853: f64, t4260: f64, t883: f64, t3116: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29976 = t6508 * t29975;
    let t29984 = t874 * t2293;
    let t29985 = t6508 * t29984;
    let t30019 = t172 * t2293;
    let t30136 = t20368 * t29975;
    let t30140 = t2366 * t29853;
    let t30204 = t4260 * t883;
    let t30208 = t3116 * t447;
    (t29976, t29984, t29985, t30019, t30136, t30140, t30204, t30208)
}
