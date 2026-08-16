//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1015/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1015(t136188: f64, t136189: f64, t25990: f64, t32067: f64, t137245: f64, t25894: f64, t32069: f64, t631: f64, t92173: f64, t1564: f64, t25878: f64, t3052: f64, t32115: f64) -> (f64, f64, f64) {
    let t144781 = t32067 * t136188 * t136189 * t25990;
    let t144786 = t92173 * t631 * t137245 * t32069 * t25894;
    let t144790 = t25878 * t1564 * t32115 * t3052;
    (t144781, t144786, t144790)
}
