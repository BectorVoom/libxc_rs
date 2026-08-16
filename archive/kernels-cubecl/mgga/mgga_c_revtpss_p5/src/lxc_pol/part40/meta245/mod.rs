//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk927;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk928;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk929;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk930;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta245<F: Float>(t33: F, t265: F, t502: F, t4560: F, t5508: F, t1113: F, t1304: F, t1469: F, t1587: F, t1711: F, t1837: F, t4186: F, t4568: F, t504: F, t57: F, t606: F, t895: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5035: F, t670: F, t93: F, t1312: F, t1518: F, t2322: F, t4246: F, t4248: F, t4292: F, t1450: F, t1907: F, t198: F, t530: F) -> (F, F, F, F, F, F) {
        let (t5509, t5516) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk927::<F>(t33, t265, t502, t4560, t5508, t1113, t1304, t1469, t1587, t1711, t1837, t4186, t4568, t504, t57, t606, t895, dens_threshold, rho1, zeta_threshold);
        let t5517 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk928::<F>(t5035, t5516);
        let t5523 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk929::<F>(t670, t93);
        let (t5528, t5532, t5536) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk930::<F>(t1312, t1518, t2322, t4246, t4248, t4292, t5523, t670, t1450, t1907, t198, t530);
    (t5509, t5517, t5523, t5528, t5532, t5536)
}
