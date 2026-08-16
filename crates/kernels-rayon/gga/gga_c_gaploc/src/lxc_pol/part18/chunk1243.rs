//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1243/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1243(t21488: f64, t314: f64, t320: f64, t3487: f64, t7291: f64, t734: f64, t2958: f64, t590: f64, t2101: f64, t10639: f64, t6058: f64, t10736: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32604 = 0.17090058289204942853e-2_f64 * t21488 * t320 * t314 * t7291 * t3487 * t734;
    let t32607 = t2958 * t7291;
    let t32608 = t590 * t32607;
    let t32610 = 0.20508069947045931422e-1_f64 * t21488 * t320 * t2101 * t32608;
    let t32613 = t590 * t10639;
    let t32615 = 0.10254034973522965711e-1_f64 * t21488 * t320 * t6058 * t32613;
    let t32616 = t590 * t10736;
    (t32604, t32608, t32610, t32613, t32615, t32616)
}
