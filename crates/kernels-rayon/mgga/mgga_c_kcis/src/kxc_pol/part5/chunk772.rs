//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 772/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk772(t1529: f64, t2043: f64, t1543: f64, t2066: f64, t4122: f64, t4291: f64) -> (f64, f64, f64) {
    let t6006 = t1529 * t2043;
    let t6008 = t1543 * t2066;
    let t6010 = t4122 * t4291;
    (t6006, t6008, t6010)
}
