//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1076/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1076(t27432: f64, t27462: f64, t27465: f64, t27477: f64, t27480: f64, t27607: f64, t27617: f64, t27648: f64, t27653: f64, t27654: f64, t27665: f64, t27668: f64, t7968: f64, t7978: f64, t7981: f64) -> f64 {
    let t27669 = 0.15476481481481481481e-2_f64 * t27432 + 0.34752604166666666667e-3_f64 * t7978 * t27648 + t27653 - 0.23168402777777777778e-3_f64 * t27654 + 0.23214722222222222222e-2_f64 * t27462 + 0.17411041666666666666e-2_f64 * t27465 - 0.34822083333333333332e-2_f64 * t27477 + 0.23214722222222222222e-2_f64 * t27480 - 0.92754700520833333334e-4_f64 * t7968 * t27617 - 0.23168402777777777778e-3_f64 * t27607 * t7981 + 0.23168402777777777778e-3_f64 * t7978 * t27665 - t27668;
    t27669
}
