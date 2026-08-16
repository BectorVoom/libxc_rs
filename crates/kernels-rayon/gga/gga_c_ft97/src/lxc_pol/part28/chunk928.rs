//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 928/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk928(t526: f64, t9439: f64, t605: f64, t9016: f64, t2178: f64, t5842: f64, t23571: f64, t50249: f64, t2101: f64, t6685: f64, t1391: f64, t9132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106698 = t526 * t9439;
    let t106729 = t9016 * t605;
    let t106761 = t2178 * t5842;
    let t106803 = t50249 * t23571;
    let t106875 = t2101 * t6685;
    let t106894 = t9132 * t1391;
    (t106698, t106729, t106761, t106803, t106875, t106894)
}
