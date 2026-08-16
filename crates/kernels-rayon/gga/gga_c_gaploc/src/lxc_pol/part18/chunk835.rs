//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 835/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk835(t7910: f64, t7948: f64, t7991: f64, t8038: f64, t2796: f64, t501: f64, t1381: f64, t997: f64, t1016: f64, t1383: f64, t2902: f64, t605: f64) -> (f64, f64, f64, f64, f64) {
    let t8040 = t7910 + t7948 + t7991 + t8038;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    let t8054 = t1016 * t1383;
    let t8057 = t2902 * t605;
    (t8040, t8042, t8045, t8054, t8057)
}
