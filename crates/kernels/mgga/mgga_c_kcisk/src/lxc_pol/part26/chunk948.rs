//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 948/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk948<F: Float>(t1203: F, t1212: F, t25838: F, t12929: F, t13110: F, t19100: F, t19102: F, t19106: F, t19632: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25607: F, t25609: F, t25612: F, t25615: F, t25618: F) -> (F, F) {
    let t25840 = t1203 * t25838 * t1212;
    let t25862 = -t13110 - 0.79148148148148148147e-2 * t12929 - 0.15829629629629629629e-1 * t19100 + 0.79148148148148148147e-2 * t19102 - t19632 + 0.23744444444444444444e-1 * t19106 + 0.39574074074074074073e-2 * t25590 - 0.19787037037037037037e-1 * t25593 + 0.71233333333333333332e-1 * t25596 - 0.47488888888888888888e-1 * t25599 - 0.11872222222222222222e-1 * t25601 - 0.10685e0 * t25604 + 0.14246666666666666666e0 * t25607 + 0.5936111111111111111e-2 * t25609 - 0.11872222222222222222e-1 * t25612 + 0.35616666666666666666e-1 * t25615 - 0.17808333333333333333e-1 * t25618;
    (t25840, t25862)
}
