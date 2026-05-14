//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 712/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk712<F: Float>(t5396: F, t760: F, t755: F, t1973: F, t5374: F, t5400: F, t10570: F, t10572: F, t10574: F, t10576: F, t10587: F, t10595: F, t10607: F, t10610: F, t10613: F, t10615: F, t10617: F, t10619: F, t10623: F, t10626: F) -> (F, F, F, F) {
    let t12017 = 1.0 / t5396 / t760;
    let t12018 = t755 * t12017;
    let t12019 = t5374 * t1973;
    let t12020 = t12019 * t5400;
    let t12037 = -0.41678000000000000001e0 * t10607 + 0.20839e0 * t10610 - 0.62517e0 * t10613 - 0.34731666666666666667e0 * t10615 + 0.20839e0 * t10617 + 0.69463333333333333335e-1 * t10619 - 0.46308888888888888889e-1 * t10623 - 0.104195e0 * t10626 - 0.103295e1 * t10587 + 0.309885e1 * t10595 - 0.68863333333333333332e0 * t10570 + 0.34431666666666666666e0 * t10572 - 0.103295e1 * t10574 + 0.51647499999999999999e0 * t10576;
    (t12018, t12019, t12020, t12037)
}
