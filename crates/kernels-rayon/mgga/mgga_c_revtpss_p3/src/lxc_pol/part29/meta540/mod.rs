//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1873;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta540(t25305: f64, t95793: f64, t25410: f64, t7419: f64, t93240: f64, t26519: f64, t93160: f64, t25372: f64, t95536: f64, t92840: f64, t7398: f64, t822: f64, t25375: f64, t95765: f64, t25411: f64, t95597: f64, t93170: f64, t95746: f64, t26446: f64, t689: f64, t887: f64, t26481: f64, t2724: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95808, t95811, t95813, t95822, t95823, t95825) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1873(t25305, t95793, t25410, t7419, t93240, t26519, t93160, t25372, t95536, t92840, t7398, t822);
        let (t95832, t95834, t95836, t95847, t95854) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1874(t25375, t95765, t25411, t95597, t93170, t95746, t26446, t689, t887, t26481, t2724, t676);
    (t95808, t95811, t95813, t95822, t95823, t95825, t95832, t95834, t95836, t95847, t95854)
}
