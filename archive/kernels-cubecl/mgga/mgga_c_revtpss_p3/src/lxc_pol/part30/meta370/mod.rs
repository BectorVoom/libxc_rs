//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta370<F: Float>(t1235: F, t12984: F, t12627: F, t225: F, t127: F, t3672: F, t371: F, t3671: F, t140: F, t3693: F, t1222: F, t1226: F, t697: F) -> (F, F, F, F, F, F, F) {
        let (t12985, t12987, t12995, t12996, t12998, t12999, t13011) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1397::<F>(t1235, t12984, t12627, t225, t127, t3672, t371, t3671, t140, t3693, t1222, t1226, t697);
    (t12985, t12987, t12995, t12996, t12998, t12999, t13011)
}
