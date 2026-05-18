//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1196/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1196<F: Float>(t14654: F, t3489: F, t27785: F, t2822: F, t27864: F, t3245: F, t8057: F, t27936: F, t7699: F, t1014: F, t27879: F, t27856: F, t7687: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96391 = t14654 * t3489;
    let t96395 = t2822 * t27785;
    let t96396 = F::new(0.14739506172839506172e-2) * t96395;
    let t96401 = t2822 * t27864;
    let t96402 = F::new(0.22109259259259259258e-2) * t96401;
    let t96412 = t3245 * t8057;
    let t96418 = F::new(0.46336805555555555556e-3) * t27936 * t7699;
    let t96427 = t1014 * t27879;
    let t96428 = F::new(0.33163888888888888888e-2) * t96427;
    let t96449 = F::new(0.46336805555555555556e-3) * t7687 * t27856;
    (t96391, t96395, t96396, t96401, t96402, t96412, t96418, t96427, t96428, t96449)
}
