//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 696/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk696(t3754: f64, t498: f64, t2642: f64, t3978: f64, t1370: f64, t1377: f64) -> (f64, f64, f64) {
    let t3979 = t498 * t3754;
    let t3980 = t3979 * t2642;
    let t3981 = t3978 * t3980;
    let t3984 = t1370 * t1377;
    (t3980, t3981, t3984)
}
