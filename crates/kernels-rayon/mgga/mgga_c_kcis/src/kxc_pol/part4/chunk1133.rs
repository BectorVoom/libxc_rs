//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1133/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1133(t13467: f64, t14347: f64, t13516: f64, t4565: f64, t1662: f64, t2952: f64, t3269: f64, t4621: f64, t934: f64, t3096: f64, t3274: f64, t1045: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14348 = t14347 * t13467;
    let t14351 = t4565 * t13516;
    let t14355 = t3269 * t1662 * t2952;
    let t14359 = t3269 * t4621 * t934;
    let t14363 = t3274 * t1662 * t3096;
    let t14367 = t3274 * t4621 * t1045;
    (t14348, t14351, t14355, t14359, t14363, t14367)
}
