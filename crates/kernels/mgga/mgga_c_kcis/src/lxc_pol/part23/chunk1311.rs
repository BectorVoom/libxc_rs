//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1311/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1311<F: Float>(t98918: F, t27601: F, t28714: F, t7968: F, t99236: F, t27560: F, t27636: F, t27638: F, t27665: F, t28853: F, t53436: F, t6176: F, t7978: F, t94656: F, t94662: F, t98915: F, t99635: F) -> F {
    let t99667 = F::cast_from(0.15476481481481481481e-2_f64) * t98918;
    let t99671 = F::cast_from(0.23168402777777777778e-3_f64) * t28714 * t27601;
    let t99676 = F::cast_from(0.30918233506944444444e-4_f64) * t7968 * t99236;
    let t99677 = -F::cast_from(0.13901041666666666667e-2_f64) * t7978 * t6176 * t27636 * t53436 - F::cast_from(0.69505208333333333334e-3_f64) * t7978 * t99635 + F::cast_from(0.23168402777777777778e-3_f64) * t28714 * t27665 - F::cast_from(0.46429444444444444443e-2_f64) * t98915 + F::cast_from(0.23214722222222222222e-2_f64) * t94656 - t99667 - F::cast_from(0.12367293402777777778e-3_f64) * t28853 * t27560 + t99671 - F::cast_from(0.69505208333333333334e-3_f64) * t28714 * t27638 - F::cast_from(0.15476481481481481481e-2_f64) * t94662 + t99676;
    t99677
}
