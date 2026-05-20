//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta386<F: Float>(t2470: F, t4480: F, t2465: F, t11008: F, t1579: F, t2771: F, t1558: F, t836: F, t231: F, t2797: F, t2782: F, t860: F) -> (F, F, F, F, F, F, F) {
        let (t14485, t14486, t14489, t14494, t14495, t14498, t14502) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1387::<F>(t2470, t4480, t2465, t11008, t1579, t2771, t1558, t836, t231, t2797, t2782, t860);
    (t14485, t14486, t14489, t14494, t14495, t14498, t14502)
}
