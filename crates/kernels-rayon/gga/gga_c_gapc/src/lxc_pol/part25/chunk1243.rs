//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1243/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1243(t11326: f64, t21642: f64, t9260: f64, t20563: f64, t5116: f64, t9061: f64, t3709: f64, t3713: f64, t5075: f64, t11450: f64, t11451: f64, t21157: f64) -> (f64, f64, f64, f64) {
    let t34695 = t11326 * t9260 * t21642;
    let t34698 = t9061 * t5116 * t20563;
    let t34701 = t3709 * t5075 * t3713;
    let t34704 = t11450 * t11451 * t21157;
    (t34695, t34698, t34701, t34704)
}
