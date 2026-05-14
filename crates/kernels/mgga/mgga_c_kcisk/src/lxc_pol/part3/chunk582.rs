//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 582/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk582<F: Float>(t4636: F, t4638: F, t4642: F, t4646: F, t4650: F, t1961: F, t1965: F, t1964: F, t760: F, t755: F, t1973: F, t1974: F, t4722: F, t4672: F, t4674: F, t4717: F, t4719: F, t4724: F, t4728: F, t4731: F, t4734: F) -> (F, F, F, F, F, F, F) {
    let t5360 = 0.22831111111111111111e-1 * t4636;
    let t5365 = t5360 + 0.11415555555555555555e-1 * t4638 - 0.11415555555555555555e-1 * t4642 + 0.34246666666666666666e-1 * t4646 - 0.17123333333333333333e-1 * t4650;
    let t5368 = t1961 * t1965;
    let t5371 = t1964 * t760;
    let t5372 = 1.0 / t5371;
    let t5373 = t755 * t5372;
    let t5374 = t1973 * t1973;
    let t5375 = t5374 * t1974;
    let t5380 = 0.68863333333333333333e0 * t4636;
    let t5387 = 0.17365833333333333333e0 * t4722;
    let t5392 = -0.17648625e1 * t4672 + 0.3529725e1 * t4674 + t5380 + 0.34431666666666666666e0 * t4638 - 0.34431666666666666667e0 * t4642 + 0.103295e1 * t4646 - 0.516475e0 * t4650 + 0.31558125e0 * t4717 + 0.6311625e0 * t4719 + t5387 + 0.13892666666666666667e0 * t4724 - 0.34731666666666666667e-1 * t4728 + 0.20839e0 * t4731 - 0.104195e0 * t4734;
    (t5365, t5368, t5372, t5373, t5374, t5375, t5392)
}
