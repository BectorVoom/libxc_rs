//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1173/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1173(t1038: f64, t2619: f64, t297: f64, t33722: f64, t7371: f64, t11745: f64, t18331: f64, t11387: f64, t7204: f64, t7557: f64, t11483: f64, t11749: f64, t2787: f64) -> (f64, f64, f64, f64) {
    let t33726 = t2619 * t33722 * t1038 * t297 * t7371;
    let t33728 = t18331 * t11745;
    let t33731 = t7204 * t11387 * t7557;
    let t33734 = t2787 * t11483 * t11749;
    (t33726, t33728, t33731, t33734)
}
