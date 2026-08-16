//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 472/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk472(t2366: f64, t475: f64, t2365: f64, t1429: f64, t1: f64, t2299: f64) -> (f64, f64, f64, f64) {
    let t2367 = t2366 * t475;
    let t2368 = t2365 * t2367;
    let t2369 = t1429 * t2368;
    let t2371 = t2299 * t1;
    (t2367, t2368, t2369, t2371)
}
