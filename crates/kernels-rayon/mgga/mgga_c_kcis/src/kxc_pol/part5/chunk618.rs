//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 618/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk618(t3245: f64, t558: f64, t1014: f64, t1460: f64, t1465: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t4114 = t3245 * t558;
    let t4115 = 0.55273148148148148147e-3_f64 * t4114;
    let t4117 = t1014 * t1460;
    let t4121 = 1.0_f64 / t1465 / t551;
    (t4114, t4115, t4117, t4121)
}
