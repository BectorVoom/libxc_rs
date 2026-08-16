//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1678;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1679;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1680;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta449<F: Float>(t624: F, t655: F, t665: F, t2339: F, t68: F, t2340: F, t2366: F, t6998: F, t1450: F, t3829: F, t555: F, t7063: F, t1032: F, t4075: F, t545: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25823, t25824, t25825, t25826, t25827, t25829, t25865, t25875) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1678::<F>(t624, t655, t665, t2339, t68, t2340, t2366, t6998, t1450, t3829, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1679::<F>(t1032, t4075, t545);
        let t25878 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1680::<F>(t25875, t25877);
        let t25894 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1681::<F>(t555, t786);
    (t25823, t25824, t25825, t25826, t25827, t25829, t25865, t25875, t25876, t25877, t25878, t25894)
}
