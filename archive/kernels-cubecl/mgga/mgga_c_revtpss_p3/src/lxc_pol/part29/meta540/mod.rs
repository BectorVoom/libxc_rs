//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1873;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta540<F: Float>(t25305: F, t95793: F, t25410: F, t7419: F, t93240: F, t26519: F, t93160: F, t25372: F, t95536: F, t92840: F, t7398: F, t822: F, t25375: F, t95765: F, t25411: F, t95597: F, t93170: F, t95746: F, t26446: F, t689: F, t887: F, t26481: F, t2724: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t95808, t95811, t95813, t95822, t95823, t95825) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1873::<F>(t25305, t95793, t25410, t7419, t93240, t26519, t93160, t25372, t95536, t92840, t7398, t822);
        let (t95832, t95834, t95836, t95847, t95854) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1874::<F>(t25375, t95765, t25411, t95597, t93170, t95746, t26446, t689, t887, t26481, t2724, t676);
    (t95808, t95811, t95813, t95822, t95823, t95825, t95832, t95834, t95836, t95847, t95854)
}
