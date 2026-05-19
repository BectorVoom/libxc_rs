//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 986/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk986<F: Float>(t1452: F, t2475: F, t531: F, t822: F, t3110: F, t317: F, t522: F, t323: F, t526: F, t8291: F, t10138: F, t534: F) -> (F, F, F, F, F, F) {
    let t12009 = t2475 * t1452;
    let t12048 = t822 * t531;
    let t12049 = F::cast_from(0.62154466893555682512e-3_f64) * t12048;
    let t12058 = F::cast_from(0.27323333333333333333e-1_f64) * t317 * t3110 * t522;
    let t12061 = F::cast_from(0.77488888888888888888e-2_f64) * t323 * t8291 * t526;
    let t12062 = t10138 * t534;
    (t12009, t12048, t12049, t12058, t12061, t12062)
}
