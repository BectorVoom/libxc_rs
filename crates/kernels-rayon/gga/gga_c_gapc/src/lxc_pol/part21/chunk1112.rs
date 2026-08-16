//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1112/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1112(t33152: f64, t3402: f64, t9934: f64, t11913: f64, t28924: f64, t11834: f64, t3137: f64, t7191: f64, t818: f64, t959: f64, t11769: f64, t9703: f64) -> (f64, f64, f64, f64) {
    let t33801 = t3402 * t33152 * t9934;
    let t33803 = t11913 * t28924;
    let t33808 = t11834 * t3137 * t818 * t959 * t7191;
    let t33810 = t11769 * t9703;
    (t33801, t33803, t33808, t33810)
}
