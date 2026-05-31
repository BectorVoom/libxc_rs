//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 492/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk492<F: Float>(t1383: F, t3579: F, t1398: F, t1375: F, t158: F, t165: F, t173: F, t3278: F, t3819: F, t3870: F, t3873: F, t3875: F, t3878: F, t3881: F, t3883: F, t3886: F, t3891: F) -> (F, F, F, F) {
    let t3894 = t1383 * t3579;
    let t3897 = t1398 * t3579;
    let t3900 = t1375 * t3579;
    let t3903 = -F::cast_from(0.672175e-5_f64) * t173 * t3870 + F::cast_from(0.9368e-2_f64) * t3873 - F::cast_from(0.3513e-2_f64) * t158 * t3875 + F::cast_from(0.1171e-2_f64) * t158 * t3878 - F::cast_from(0.26416666666666666666e-2_f64) * t3881 + F::cast_from(0.7925e-3_f64) * t165 * t3883 - F::cast_from(0.52833333333333333333e-3_f64) * t165 * t3886 - F::cast_from(0.23911438650126355246e-1_f64) * t3819 * t3278 + F::cast_from(0.15538616723388920628e-3_f64) * t3891 * t3278 - F::cast_from(0.1585e-2_f64) * t165 * t3894 - F::cast_from(0.10082625e-4_f64) * t173 * t3897 + F::cast_from(0.7026e-2_f64) * t158 * t3900;
    (t3894, t3897, t3900, t3903)
}
