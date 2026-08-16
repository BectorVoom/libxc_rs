//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2749/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2749<F: Float>(t41279: F, t5499: F, t12945: F, t4205: F, t46208: F, t4194: F, t5398: F, t607: F, t750: F, t46217: F, t13130: F, t32: F, t5519: F) -> (F, F, F, F, F, F, F) {
    let t57959 = F::cast_from(12.0_f64) * t41279 * t5499;
    let t57960 = t4205 * t12945;
    let t57961 = F::cast_from(8.0_f64) * t57960;
    let t57962 = F::cast_from(0.20508037716432813315e4_f64) * t46208;
    let t57965 = t4194 * t750 * t5398 * t607;
    let t57966 = F::cast_from(24.0_f64) * t57965;
    let t57970 = F::cast_from(16.0_f64) * t46217;
    let t57972 = F::cast_from(8.0_f64) * t4205 * t13130;
    let t57973 = t32 * t5519;
    (t57959, t57961, t57962, t57966, t57970, t57972, t57973)
}
