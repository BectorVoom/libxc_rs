//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1339/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1339(t10558: f64, t177: f64, t762: f64, t150: f64, t190: f64, t39854: f64, t2491: f64, t2495: f64, t39871: f64, t760: f64, t10433: f64, t2398: f64) -> (f64, f64, f64, f64, f64) {
    let t40108 = t10558 * t177 * t762;
    let t40109 = 0.23392894490538584828e1_f64 * t40108;
    let t40111 = t150 * t39854 * t190;
    let t40113 = t2491 * t39871 * t2495;
    let t40115 = 0.51947577317044391277e2_f64 * t760 * t40113;
    let t40117 = 16.0_f64 * t2398 * t10433;
    (t40109, t40111, t40113, t40115, t40117)
}
