//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 948/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk948<F: Float>(t3897: F, t970: F, t1186: F, t12952: F, t12957: F, t1383: F, t1398: F, t1375: F, t14014: F, t14016: F, t14019: F, t14022: F, t14025: F, t14027: F, t14029: F, t158: F, t165: F, t173: F) -> F {
    let t14031 = t970 * t3897;
    let t14033 = t1186 * t12952;
    let t14036 = t1383 * t12957;
    let t14039 = t1398 * t12957;
    let t14042 = t1375 * t12957;
    let t14045 = F::cast_from(0.26416666666666666666e-2_f64) * t14014 + F::cast_from(0.7925e-3_f64) * t165 * t14016 - F::cast_from(0.17611111111111111111e-3_f64) * t165 * t14019 - F::cast_from(0.7026e-2_f64) * t158 * t14022 + F::cast_from(0.79249999999999999999e-2_f64) * t14025 + F::cast_from(0.10566666666666666666e-1_f64) * t14027 + F::cast_from(0.117630625e-3_f64) * t14029 + F::cast_from(0.70578375e-4_f64) * t14031 + F::cast_from(0.317e-2_f64) * t165 * t14033 + F::cast_from(0.4755e-2_f64) * t165 * t14036 + F::cast_from(0.30247875e-4_f64) * t173 * t14039 - F::cast_from(0.21078e-1_f64) * t158 * t14042;
    t14045
}
