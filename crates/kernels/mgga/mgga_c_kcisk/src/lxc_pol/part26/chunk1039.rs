//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1039/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1039<F: Float>(t12888: F, t7802: F, t1210: F, t12929: F, t14831: F, t19100: F, t19102: F, t19106: F, t21720: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25607: F, t25609: F, t25612: F, t25615: F, t25618: F) -> (F, F) {
    let t27642 = t7802 * t12888;
    let t27643 = t27642 * t1210;
    let t27661 = -t14831 - 0.76103703703703703703e-2 * t12929 - 0.1522074074074074074e-1 * t19100 + 0.761037037037037037e-2 * t19102 - t21720 + 0.2283111111111111111e-1 * t19106 + 0.3805185185185185185e-2 * t25590 - 0.19025925925925925925e-1 * t25593 + 0.68493333333333333331e-1 * t25596 - 0.4566222222222222222e-1 * t25599 - 0.11415555555555555555e-1 * t25601 - 0.10274e0 * t25604 + 0.13698666666666666666e0 * t25607 + 0.57077777777777777777e-2 * t25609 - 0.11415555555555555555e-1 * t25612 + 0.34246666666666666666e-1 * t25615 - 0.17123333333333333333e-1 * t25618;
    (t27643, t27661)
}
