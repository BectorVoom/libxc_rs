//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 475/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk475(t2334: f64, t475: f64, t1445: f64, t2345: f64, t188: f64, t2304: f64, t1645: f64, t494: f64) -> (f64, f64, f64, f64, f64) {
    let t2378 = t2334 * t475;
    let t2379 = t1445 * t2378;
    let t2382 = t1445 * t2345;
    let t2385 = t188 * t2304;
    let t2386 = t1645 * t494;
    (t2378, t2379, t2382, t2385, t2386)
}
