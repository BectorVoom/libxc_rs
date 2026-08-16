//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1403/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1403<F: Float>(t26264: F, t26252: F, t26258: F, t26326: F, t26328: F, t26330: F, t26332: F, t26351: F, t26354: F, t26358: F, t27950: F, t26314: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F) -> (F, F) {
    let t27951 = F::cast_from(0.12819753086419753086e4_f64) * t26264;
    let t27959 = F::cast_from(0.10769135802469135803e4_f64) * t26252 + F::cast_from(0.96922222222222222221e4_f64) * t26258 + t27950 + t27951 - F::cast_from(0.19384444444444444445e4_f64) * t26326 - F::cast_from(0.12922962962962962963e4_f64) * t26328 - F::cast_from(0.41955555555555555556e3_f64) * t26351 + F::cast_from(0.41955555555555555555e3_f64) * t26354 + F::cast_from(0.38768888888888888889e4_f64) * t26330 + F::cast_from(0.30153580246913580247e4_f64) * t26332 + F::cast_from(0.93234567901234567903e3_f64) * t26358;
    let t27972 = -F::cast_from(0.21538271604938271605e4_f64) * t26339 - F::cast_from(0.72691666666666666667e3_f64) * t26343 - F::cast_from(0.52444444444444444446e3_f64) * t26363 - F::cast_from(0.17481481481481481482e3_f64) * t26365 + F::cast_from(0.20977777777777777778e3_f64) * t26367 + F::cast_from(0.932345679012345679e2_f64) * t26369 + F::cast_from(0.96922222222222222224e3_f64) * t26314 + F::cast_from(0.10488888888888888889e4_f64) * t26372 - F::cast_from(0.81580246913580246914e2_f64) * t26376 - F::cast_from(0.78666666666666666667e2_f64) * t26379 - F::cast_from(0.20977777777777777778e3_f64) * t26382 + F::cast_from(0.62933333333333333332e3_f64) * t26385;
    (t27959, t27972)
}
