//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 967/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk967(t8557: f64, t8567: f64, t8571: f64, t8576: f64, t8579: f64, t8585: f64, t8682: f64, t8691: f64, t8742: f64, t8901: f64, t8903: f64, t1214: f64, t2905: f64) -> (f64, f64) {
    let t9265 = -t8742 + t8901 - t8567 + t8571 + t8576 + t8579 - t8585 + t8682 + t8691 + t8903 - t8557;
    let t9266 = t2905 * t1214;
    (t9265, t9266)
}
