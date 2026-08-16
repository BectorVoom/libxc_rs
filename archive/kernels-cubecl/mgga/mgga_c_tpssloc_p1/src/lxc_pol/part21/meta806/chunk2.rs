//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2800/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2800<F: Float>(t5555: F, t9541: F, t210: F, t214: F, t2571: F, t41200: F, t46782: F, t46788: F, t46790: F, t46793: F, t46796: F, t46802: F, t46806: F, t46819: F, t46828: F, t46830: F, t46836: F, t58090: F) -> F {
    let t59195 = t9541 * t5555;
    let t59197 = -F::cast_from(0.2111111111111111111e-1_f64) * t46782 - t41200 + F::cast_from(0.77777777777777777775e-2_f64) * t46788 + F::cast_from(0.11234567901234567901e0_f64) * t46790 + F::cast_from(0.15555555555555555555e0_f64) * t46793 + F::cast_from(0.6333333333333333333e-1_f64) * t46796 + F::cast_from(0.19999999999999999999e-1_f64) * t46802 + F::cast_from(0.55555555555555555553e-3_f64) * t46806 - F::cast_from(0.99999999999999999996e-2_f64) * t46819 - F::cast_from(0.49999999999999999998e-2_f64) * t46828 - F::cast_from(0.46666666666666666664e-1_f64) * t46830 - F::cast_from(0.23333333333333333332e-1_f64) * t46836 + F::cast_from(0.99999999999999999996e-2_f64) * t2571 * t210 * t214 * t58090 - F::cast_from(0.12962962962962962962e-1_f64) * t59195;
    t59197
}
