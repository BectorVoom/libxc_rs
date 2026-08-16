//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 910/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk910(t2855: f64, t6334: f64, t1021: f64, t1020: f64, t1121: f64, t6486: f64, t1022: f64, t9589: f64, t1092: f64, t1133: f64, t1131: f64, t3227: f64) -> (f64, f64, f64, f64, f64) {
    let t19605 = t2855 * t6334;
    let t19606 = t1021 * t19605;
    let t19607 = t1020 * t19606;
    let t19609 = t6486 * t1121;
    let t19610 = t1022 * t19609;
    let t19611 = t9589 * t19610;
    let t19612 = t1092 * t19611;
    let t19614 = t6486 * t1133;
    let t19615 = t1131 * t19614;
    let t19616 = t3227 * t19615;
    (t19607, t19609, t19612, t19614, t19616)
}
