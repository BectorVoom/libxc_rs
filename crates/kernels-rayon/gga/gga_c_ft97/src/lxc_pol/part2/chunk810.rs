//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 810/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk810(t11437: f64, t12724: f64, t12723: f64, t1651: f64, t3445: f64, t2221: f64, t1643: f64, t9115: f64, t2157: f64, t920: f64, t2211: f64, t2210: f64) -> (f64, f64, f64, f64) {
    let t12725 = t12724 * t11437;
    let t12726 = t12723 * t12725;
    let t12729 = t3445 * t1651;
    let t12730 = t2221 * t12729;
    let t12733 = t3445 * t1643;
    let t12734 = t9115 * t12733;
    let t12737 = t920 * t2157;
    let t12738 = t2211 * t12737;
    let t12739 = t2210 * t12738;
    (t12726, t12730, t12734, t12739)
}
