//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta851 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta851<F: Float>(t2349: F, t656: F, t10227: F, t97: F, t10241: F, t105: F, t4273: F, t588: F, t2289: F, t4288: F, t13455: F, t625: F, t14619: F, t750: F, t4398: F, t9372: F, t1469: F, t2608: F, t4401: F, t606: F, t14425: F, t705: F, t39454: F, t9387: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t49774, t49777, t49787, t49804, t49817, t49819) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2991::<F>(t2349, t656, t10227, t97, t10241, t105, t4273, t588, t2289, t4288, t13455, t625);
        let (t49864, t49866, t49876, t49880, t49887, t49897) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2992::<F>(t14619, t750, t4398, t9372, t1469, t2608, t4401, t606, t14425, t705, t39454, t9387);
    (t49774, t49777, t49787, t49804, t49817, t49819, t49864, t49866, t49876, t49880, t49887, t49897)
}
