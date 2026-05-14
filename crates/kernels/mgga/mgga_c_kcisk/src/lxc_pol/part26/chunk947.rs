//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 947/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk947<F: Float>(t12929: F, t19100: F, t19106: F, t19485: F, t19645: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25609: F, t25618: F, t25696: F, t25716: F, t25719: F, t25722: F, t25725: F, t25728: F, t25731: F, t25812: F, t25836: F) -> (F,) {
    let t25838 = -t19645 + 0.36793333333333333333e-1 * t19485 - 0.13418888888888888889e0 * t12929 - 0.26837777777777777779e0 * t19100 + 0.40256666666666666668e0 * t19106 + 0.67094444444444444443e-1 * t25590 - 0.20128333333333333333e0 * t25601 + 0.10064166666666666667e0 * t25609 - 0.301925e0 * t25618 - 0.11038e0 * t25696 + t25812 - 0.36793333333333333333e-1 * t25716 - 0.11038e0 * t25719 - 0.49671e0 * t25722 + 0.66228e0 * t25725 + 0.16557e0 * t25728 - 0.27595e-1 * t25731 - 0.33547222222222222222e0 * t25593 + 0.12077e1 * t25596 - 0.80513333333333333332e0 * t25599 - 0.181155e1 * t25604 + t25836;
    (t25838,)
}
