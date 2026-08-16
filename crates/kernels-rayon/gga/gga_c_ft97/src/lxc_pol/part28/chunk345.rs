//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 345/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk345(t1690: f64, t5552: f64, t5555: f64, t428: f64, t5546: f64, t1701: f64, t408: f64, t6: f64) -> (f64, f64, f64, f64) {
    let t5557 = t1690 * t5552 * t5555;
    let t5560 = t5546 * t428;
    let t5561 = t1701 * t5560;
    let t5566 = t408 * t6;
    (t5557, t5560, t5561, t5566)
}
