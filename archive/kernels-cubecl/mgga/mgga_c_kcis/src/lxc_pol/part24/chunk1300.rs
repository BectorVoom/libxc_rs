//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1300/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1300<F: Float>(t27856: F, t8030: F, t101355: F, t26781: F, t14443: F, t28983: F, t7703: F, t100660: F, t101319: F, t2173: F, t26728: F, t26748: F, t27919: F, t27936: F, t28948: F, t28952: F, t29007: F, t8034: F, t93779: F, t96121: F, t96124: F) -> (F, F) {
    let t101410 = t8030 * t27856;
    let t101434 = t26781 * t101355;
    let t101437 = t7703 * t14443 * t28983;
    let t101445 = F::cast_from(0.92754700520833333333e-4_f64) * t26728 * t28948 + F::cast_from(0.13901041666666666667e-2_f64) * t27936 * t8034 + F::cast_from(0.13901041666666666667e-2_f64) * t8030 * t27919 + F::cast_from(0.49512459138020833333e-4_f64) * t93779 * t28952 - F::cast_from(0.61890573922526041667e-5_f64) * t101434 + F::cast_from(0.15445601851851851852e-3_f64) * t101437 - F::cast_from(0.7369753086419753086e-3_f64) * t96121 - F::cast_from(0.23168402777777777778e-3_f64) * t26748 * t29007 - t96124 + F::cast_from(0.69505208333333333333e-3_f64) * t2173 * t101319 - F::cast_from(0.13265555555555555555e-1_f64) * t100660;
    (t101410, t101445)
}
