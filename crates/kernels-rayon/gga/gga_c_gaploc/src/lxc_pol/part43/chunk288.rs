//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 288/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk288(t2334: f64, t447: f64, t1064: f64, t550: f64, t1365: f64, t1570: f64, t169: f64) -> (f64, f64, f64, f64, f64) {
    let t2335 = t2334 * t447;
    let t2336 = t1064 * t2335;
    let t2339 = t550 * t2334;
    let t2340 = t1365 * t2339;
    let t2343 = t1570 * t169;
    (t2335, t2336, t2339, t2340, t2343)
}
