//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1182/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1182(t1030: f64, t33303: f64, t3123: f64, t11483: f64, t1688: f64, t1804: f64, t21183: f64, t11485: f64, t33643: f64, t633: f64, t11496: f64, t185: f64, t33712: f64) -> (f64, f64, f64, f64, f64) {
    let t34681 = t1030 * t33303;
    let t34682 = t34681 * t3123;
    let t34686 = t1804 * t11483 * t1688 * t21183;
    let t34689 = t633 * t33643 * t11485;
    let t34692 = t185 * t33712 * t11496;
    (t34681, t34682, t34686, t34689, t34692)
}
