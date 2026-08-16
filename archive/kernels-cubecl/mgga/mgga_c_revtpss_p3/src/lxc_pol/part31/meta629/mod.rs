//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta629<F: Float>(t25526: F, t4820: F, t15769: F, t25522: F, t15687: F, t25515: F, t3317: F, t25525: F, t4878: F, t27450: F, t3173: F, t16035: F, t25580: F) -> (F, F, F, F, F, F, F) {
        let (t100048, t100051, t100054, t100055, t100074, t100078, t100092) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083::<F>(t25526, t4820, t15769, t25522, t15687, t25515, t3317, t25525, t4878, t27450, t3173, t16035, t25580);
    (t100048, t100051, t100054, t100055, t100074, t100078, t100092)
}
