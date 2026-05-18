//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1107/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1107<F: Float>(t3275: F, t3276: F, t39286: F, t10943: F, t11603: F, t10918: F, t3579: F, t495: F, t797: F, t10615: F, t11559: F, t2333: F, t2847: F) -> (F, F, F, F, F) {
    let t39289 = F::new(5.0) / F::new(16.0) * t3275 * t3276 * t39286;
    let t39290 = t10943 * t11603;
    let t39295 = t3579 * t495 * t10918 * t797 / F::new(2.0);
    let t39298 = F::new(5.0) / F::new(8.0) * t3275 * t10615 * t11559;
    let t39299 = t2333 * t2847;
    (t39289, t39290, t39295, t39298, t39299)
}
