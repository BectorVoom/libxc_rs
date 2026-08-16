//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 865/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk865(t3010: f64, t4614: f64, t1: f64, t2925: f64, t106: f64, t316: f64) -> (f64, f64, f64, f64) {
    let t8629 = t4614 * t3010;
    let t8632 = t2925 * t1;
    let t8633 = t8632 * t106;
    let t8634 = t8633 * t316;
    (t8629, t8632, t8633, t8634)
}
