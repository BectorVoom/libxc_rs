//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 526/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk526(t2269: f64, t362: f64, t2263: f64, t1428: f64, t176: f64, t998: f64) -> (f64, f64, f64, f64) {
    let t4039 = t362 * t2269;
    let t4044 = t362 * t2263;
    let t4053 = t176 * t1428;
    let t4054 = t4053 * t998;
    (t4039, t4044, t4053, t4054)
}
