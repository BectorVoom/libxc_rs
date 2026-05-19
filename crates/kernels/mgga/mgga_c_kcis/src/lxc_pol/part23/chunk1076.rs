//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1076/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1076<F: Float>(t27432: F, t27462: F, t27465: F, t27477: F, t27480: F, t27607: F, t27617: F, t27648: F, t27653: F, t27654: F, t27665: F, t27668: F, t7968: F, t7978: F, t7981: F) -> F {
    let t27669 = F::cast_from(0.15476481481481481481e-2_f64) * t27432 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t27648 + t27653 - F::cast_from(0.23168402777777777778e-3_f64) * t27654 + F::cast_from(0.23214722222222222222e-2_f64) * t27462 + F::cast_from(0.17411041666666666666e-2_f64) * t27465 - F::cast_from(0.34822083333333333332e-2_f64) * t27477 + F::cast_from(0.23214722222222222222e-2_f64) * t27480 - F::cast_from(0.92754700520833333334e-4_f64) * t7968 * t27617 - F::cast_from(0.23168402777777777778e-3_f64) * t27607 * t7981 + F::cast_from(0.23168402777777777778e-3_f64) * t7978 * t27665 - t27668;
    t27669
}
