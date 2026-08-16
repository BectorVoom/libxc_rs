//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1300/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1300(t27856: f64, t8030: f64, t101355: f64, t26781: f64, t14443: f64, t28983: f64, t7703: f64, t100660: f64, t101319: f64, t2173: f64, t26728: f64, t26748: f64, t27919: f64, t27936: f64, t28948: f64, t28952: f64, t29007: f64, t8034: f64, t93779: f64, t96121: f64, t96124: f64) -> (f64, f64) {
    let t101410 = t8030 * t27856;
    let t101434 = t26781 * t101355;
    let t101437 = t7703 * t14443 * t28983;
    let t101445 = 0.92754700520833333333e-4_f64 * t26728 * t28948 + 0.13901041666666666667e-2_f64 * t27936 * t8034 + 0.13901041666666666667e-2_f64 * t8030 * t27919 + 0.49512459138020833333e-4_f64 * t93779 * t28952 - 0.61890573922526041667e-5_f64 * t101434 + 0.15445601851851851852e-3_f64 * t101437 - 0.7369753086419753086e-3_f64 * t96121 - 0.23168402777777777778e-3_f64 * t26748 * t29007 - t96124 + 0.69505208333333333333e-3_f64 * t2173 * t101319 - 0.13265555555555555555e-1_f64 * t100660;
    (t101410, t101445)
}
