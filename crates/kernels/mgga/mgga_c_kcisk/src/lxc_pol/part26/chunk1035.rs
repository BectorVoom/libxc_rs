//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1035/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1035<F: Float>(t12929: F, t19100: F, t19106: F, t19485: F, t21771: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25609: F, t25618: F, t25696: F, t25716: F, t25719: F, t25722: F, t25725: F, t25728: F, t25731: F, t27545: F, t27569: F) -> (F,) {
    let t27571 = -t21771 + 0.4630888888888888889e-1 * t19485 - 0.22954444444444444444e0 * t12929 - 0.45908888888888888888e0 * t19100 + 0.68863333333333333332e0 * t19106 + 0.11477222222222222222e0 * t25590 - 0.34431666666666666667e0 * t25601 + 0.17215833333333333333e0 * t25609 - 0.516475e0 * t25618 - 0.13892666666666666667e0 * t25696 + t27545 - 0.46308888888888888889e-1 * t25716 - 0.13892666666666666667e0 * t25719 - 0.62517e0 * t25722 + 0.83356e0 * t25725 + 0.20839e0 * t25728 - 0.34731666666666666667e-1 * t25731 - 0.57386111111111111112e0 * t25593 + 0.20659e1 * t25596 - 0.13772666666666666667e1 * t25599 - 0.309885e1 * t25604 + t27569;
    (t27571,)
}
