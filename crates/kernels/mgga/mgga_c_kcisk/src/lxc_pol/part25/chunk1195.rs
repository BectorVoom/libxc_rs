//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1195/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1195<F: Float>(t33220: F, t7609: F, t33219: F, t33196: F, t33258: F, t34162: F, t34165: F, t34168: F, t34171: F, t34175: F, t34178: F, t34496: F, t34534: F, t34537: F, t34548: F, t9740: F, t9995: F) -> (F, F, F) {
    let t34551 = t33220 * t7609;
    let t34552 = t33219 * t34551;
    let t34557 = 0.17361111111111111111e-2 * t9740 * t34534 + 0.17361111111111111111e-2 * t34537 + 0.34822083333333333332e-2 * t34162 + 0.46429444444444444443e-2 * t34165 - 0.30952962962962962962e-2 * t34168 + 0.11607361111111111111e-2 * t34171 - 0.17411041666666666666e-2 * t34175 + 0.11607361111111111111e-2 * t34178 + 0.20104166666666666667e-2 * t33258 * t9995 + 0.17361111111111111111e-2 * t9740 * t34548 + 0.17361111111111111111e-2 * t9740 * t34552 + 0.67013888888888888888e-3 * t33196 * t34496;
    (t34551, t34552, t34557)
}
