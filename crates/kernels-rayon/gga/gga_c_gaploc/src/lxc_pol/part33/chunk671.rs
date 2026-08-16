//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 671/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk671(t121: f64, t2154: f64, t1645: f64, t1881: f64, t120: f64, t2109: f64, t824: f64, t1: f64, t830: f64, t106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6081 = t2154 * t121;
    let t6096 = t1645 * t1881;
    let t6099 = t2109 * t120;
    let t6100 = t6099 * t824;
    let t6109 = t830 * t1;
    let t6110 = t6109 * t106;
    (t6081, t6096, t6099, t6100, t6109, t6110)
}
