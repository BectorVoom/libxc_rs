//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 545/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk545(t1021: f64, t5013: f64, t1092: f64, t1769: f64, t2861: f64, t1767: f64, t2855: f64, t1096: f64, t1775: f64, t1094: f64, t1747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5014 = t1021 * t5013;
    let t5015 = t1092 * t5014;
    let t5017 = t2861 * t1769;
    let t5019 = t2855 * t1767;
    let t5020 = t1096 * t5019;
    let t5021 = t1092 * t5020;
    let t5023 = t2861 * t1775;
    let t5025 = t1747 * t1094;
    (t5014, t5015, t5017, t5019, t5020, t5021, t5023, t5025)
}
