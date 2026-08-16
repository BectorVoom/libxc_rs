//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 138/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk138(t242: f64, t245: f64, t248: f64, t255: f64) -> (f64, f64, f64) {
    let t401 = 0.705945e1_f64 * t245 + 0.1549425e1_f64 * t242 + 0.420775e0_f64 * t248 + 0.1562925e0_f64 * t255;
    let t404 = 1.0_f64 + 0.32164683177870697974e2_f64 / t401;
    let t405 = f64::ln(t404);
    (t401, t404, t405)
}
