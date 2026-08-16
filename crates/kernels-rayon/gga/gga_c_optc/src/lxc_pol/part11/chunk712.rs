//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 712/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk712(t7256: f64, t7856: f64, t2263: f64, t896: f64, t2595: f64, t7298: f64, t141: f64, t508: f64) -> (f64, f64, f64, f64) {
    let t7857 = t7856 * t7256;
    let t7865 = t896 * t2263;
    let t7870 = t2595 * t7298;
    let t7878 = t141 * t508;
    (t7857, t7865, t7870, t7878)
}
