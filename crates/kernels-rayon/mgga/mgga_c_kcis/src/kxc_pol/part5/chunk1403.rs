//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1403/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1403(t12844: f64, t7417: f64, t4439: f64, t68: f64, t7402: f64, t610: f64, t4425: f64, t7425: f64, t1599: f64, t4455: f64, t7492: f64, t1610: f64) -> (f64, f64, f64, f64) {
    let t23154 = t12844 * t7417;
    let t23155 = t4439 * t23154;
    let t23157 = t7402 * t68;
    let t23158 = t610 * t23157;
    let t23163 = t4425 * t7425;
    let t23164 = t1599 * t23163;
    let t23167 = t4455 * t7492;
    let t23168 = t23167 * t1610;
    (t23155, t23158, t23164, t23168)
}
