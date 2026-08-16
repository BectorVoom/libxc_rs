//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1078/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1078(t1181: f64, t20311: f64, t7351: f64, t7426: f64, t1165: f64, t21118: f64, t8600: f64, t5209: f64, t7822: f64, t7637: f64, t8555: f64, t12610: f64, t1426: f64, t2297: f64, t598: f64) -> (f64, f64, f64, f64, f64) {
    let t35194 = t7426 * t1181 * t7351 * t20311;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35200 = t7822 * t5209;
    let t35204 = t7637 * t8555;
    let t35208 = t598 * t1426 * t12610 * t2297;
    (t35194, t35198, t35200, t35204, t35208)
}
