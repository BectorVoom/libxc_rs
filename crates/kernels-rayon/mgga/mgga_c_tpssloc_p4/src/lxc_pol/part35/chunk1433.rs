//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1433/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1433(t104911: f64, t104953: f64, t104958: f64, t106826: f64, t106829: f64, t106853: f64, t106862: f64, t2110: f64, t24514: f64, t27341: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t7432: f64, t7975: f64, t7978: f64, t96045: f64) -> f64 {
    let t109004 = t104911 * t106853 - 15.0_f64 * t96045 * t27961 - 15.0_f64 * t24514 * t106826 + 5.0_f64 / 2.0_f64 * t104953 * t7432 + 5.0_f64 * t104958 * t7432 + 5.0_f64 * t27341 * t27972 + 5.0_f64 / 2.0_f64 * t27341 * t27976 + t106862 * t2110 + t106829 * t2110 + 2.0_f64 * t27966 * t7975 + 2.0_f64 * t27966 * t7978;
    t109004
}
