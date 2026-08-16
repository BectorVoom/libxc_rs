//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1412/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1412<F: Float>(t28017: F, t5371: F, t106951: F, t1401: F, t1851: F, t5456: F, t1873: F, t16524: F, t28896: F, t28899: F, t33185: F, t20162: F, t7467: F) -> (F, F, F, F, F, F, F) {
    let t107568 = F::cast_from(0.405e2_f64) * t5371 * t28017;
    let t107570 = F::cast_from(0.135e2_f64) * t1401 * t106951;
    let t107571 = t1851 * t5456;
    let t107573 = F::cast_from(81.0_f64) * t107571 * t1873;
    let t107575 = F::cast_from(162.0_f64) * t16524 * t28896;
    let t107577 = F::cast_from(81.0_f64) * t16524 * t28899;
    let t107579 = F::cast_from(81.0_f64) * t33185 * t28899;
    let t107581 = F::cast_from(0.405e2_f64) * t20162 * t7467;
    (t107568, t107570, t107573, t107575, t107577, t107579, t107581)
}
