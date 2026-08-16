//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1069/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1069(t1403: f64, t35275: f64, t681: f64, t35262: f64, t27929: f64, t7437: f64, t109755: f64, t1449: f64, t35617: f64, t8392: f64, t2567: f64, t7484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t151421 = t1403 * t681 * t35275;
    let t151426 = t1403 * t681 * t35262;
    let t151428 = t7437 * t27929;
    let t151430 = t109755 * t1449;
    let t151461 = t8392 * t35617;
    let t151471 = t2567 * t7484;
    (t151421, t151426, t151428, t151430, t151461, t151471)
}
