//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1332/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1332(t1092: f64, t1800: f64, t27763: f64, t3316: f64, t26685: f64, t96480: f64, t26692: f64, t26748: f64, t27775: f64, t27780: f64, t27808: f64, t7703: f64, t93785: f64, t95537: f64, t95769: f64, t96478: f64, t96482: f64, t96486: f64, t96489: f64) -> (f64, f64) {
    let t96498 = t1092 * t27763 * t1800 * t3316;
    let t96504 = t26685 * t96480;
    let t96506 = -0.88437037037037037034e-2_f64 * t96478 - t96482 + 0.41703125000000000001e-2_f64 * t7703 * t95537 + 0.24872916666666666666e-2_f64 * t96486 - 0.24872916666666666666e-2_f64 * t96489 - 0.23168402777777777778e-3_f64 * t93785 + 0.74138888888888888889e-2_f64 * t26692 * t27775 + 0.37069444444444444444e-2_f64 * t26692 * t27780 + 0.49745833333333333332e-2_f64 * t96498 - 0.27802083333333333334e-2_f64 * t26748 * t27808 - 0.27802083333333333334e-2_f64 * t7703 * t95769 - 0.61836467013888888888e-4_f64 * t96504;
    (t96498, t96506)
}
