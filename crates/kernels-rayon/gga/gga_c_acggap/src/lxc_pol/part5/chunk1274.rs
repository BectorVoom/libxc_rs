//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1274/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1274(t4389: f64, t5859: f64, t1181: f64, t12991: f64, t4347: f64, t530: f64, t4396: f64, t6332: f64, t12930: f64, t1761: f64, t3409: f64, t5807: f64) -> (f64, f64, f64, f64, f64) {
    let t23568 = t4389 * t5859;
    let t23572 = t12991 * t1181 * t530 * t4347;
    let t23574 = t4396 * t6332;
    let t23584 = t12930 * t1761;
    let t23586 = t3409 * t5807;
    (t23568, t23572, t23574, t23584, t23586)
}
