//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1011/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1011(t12328: f64, t12330: f64, t12332: f64, t12334: f64, t12336: f64, t12338: f64, t12341: f64, t12344: f64, t12345: f64, t12348: f64, t12434: f64, t12572: f64) -> f64 {
    let t12573 = -t12328 + t12330 + t12332 - t12334 + t12336 - t12338 + t12341 - t12344 + t12345 - t12348 + t12434;
    let t12574 = t12572 + t12573;
    t12574
}
