//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1311/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1311(t98918: f64, t27601: f64, t28714: f64, t7968: f64, t99236: f64, t27560: f64, t27636: f64, t27638: f64, t27665: f64, t28853: f64, t53436: f64, t6176: f64, t7978: f64, t94656: f64, t94662: f64, t98915: f64, t99635: f64) -> f64 {
    let t99667 = 0.15476481481481481481e-2_f64 * t98918;
    let t99671 = 0.23168402777777777778e-3_f64 * t28714 * t27601;
    let t99676 = 0.30918233506944444444e-4_f64 * t7968 * t99236;
    let t99677 = -0.13901041666666666667e-2_f64 * t7978 * t6176 * t27636 * t53436 - 0.69505208333333333334e-3_f64 * t7978 * t99635 + 0.23168402777777777778e-3_f64 * t28714 * t27665 - 0.46429444444444444443e-2_f64 * t98915 + 0.23214722222222222222e-2_f64 * t94656 - t99667 - 0.12367293402777777778e-3_f64 * t28853 * t27560 + t99671 - 0.69505208333333333334e-3_f64 * t28714 * t27638 - 0.15476481481481481481e-2_f64 * t94662 + t99676;
    t99677
}
