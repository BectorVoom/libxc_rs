//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 286/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk286(t1035: f64, t1045: f64, t1027: f64, t317: f64, t319: f64, t334: f64, t240: f64, t41: f64) -> (f64, f64, f64, f64) {
    let t1046 = t1035 * t1045;
    let t1050 = 0.11955719325063177623e-1_f64 * t1027;
    let t1055 = 0.3513e-2_f64 * t317 * t334 * t319;
    let t1056 = t41 * t240;
    (t1046, t1050, t1055, t1056)
}
