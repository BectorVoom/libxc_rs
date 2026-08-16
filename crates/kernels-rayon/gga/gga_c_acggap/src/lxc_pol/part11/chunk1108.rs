//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1108/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1108(t7637: f64, t8555: f64, t12610: f64, t1426: f64, t2297: f64, t598: f64, t1967: f64, t8549: f64, t30219: f64, t8515: f64, t4680: f64, t7575: f64, t8514: f64) -> (f64, f64, f64, f64, f64) {
    let t35204 = t7637 * t8555;
    let t35208 = t598 * t1426 * t12610 * t2297;
    let t35210 = t1967 * t8549;
    let t35211 = 0.94344276868812456204e-2_f64 * t35210;
    let t35212 = t30219 * t8515;
    let t35213 = 0.21437009059034868486e-2_f64 * t35212;
    let t35215 = t7575 * t4680 * t8514;
    (t35204, t35208, t35211, t35213, t35215)
}
