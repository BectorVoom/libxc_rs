//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta677<F: Float>(t1936: F, t85360: F, t18245: F, t7002: F, t109150: F, t109153: F, t30138: F, t13426: F, t7741: F, t18227: F, t28042: F, t4248: F) -> (F, F, F, F, F, F, F, F) {
        let (t109224, t109226, t109228, t109230, t109233, t109235, t109237, t109239) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2209::<F>(t1936, t85360, t18245, t7002, t109150, t109153, t30138, t13426, t7741, t18227, t28042, t4248);
    (t109224, t109226, t109228, t109230, t109233, t109235, t109237, t109239)
}
