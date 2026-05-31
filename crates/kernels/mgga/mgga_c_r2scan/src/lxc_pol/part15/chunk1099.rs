//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1099/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1099<F: Float>(t3275: F, t3276: F, t39178: F, t6897: F, t910: F, t2330: F, t3262: F, t3263: F, t11622: F, t37271: F, t3261: F, t5086: F, t97: F) -> (F, F, F, F) {
    let t39181 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3275 * t3276 * t39178;
    let t39182 = t6897 * t910;
    let t39183 = t39182 * t2330;
    let t39186 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3263 * t39183;
    let t39188 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t37271 * t11622;
    let t39190 = t97 * t3261 * t5086;
    (t39181, t39186, t39188, t39190)
}
