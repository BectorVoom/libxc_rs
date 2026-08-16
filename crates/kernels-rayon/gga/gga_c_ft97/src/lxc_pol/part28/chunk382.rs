//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 382/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk382(t139: f64, t5790: f64, t1701: f64, t554: f64, t5546: f64, t527: f64, t5784: f64, t549: f64, t6: f64, t8: f64, t2001: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5791 = t5790 * t139;
    let t5797 = t1701 * t5546 * t554;
    let t5802 = t527 * t5784;
    let t5811 = t549 * t6;
    let t5812 = t5811 * t8;
    let t5813 = t2001 * t5812;
    (t5791, t5797, t5802, t5811, t5812, t5813)
}
