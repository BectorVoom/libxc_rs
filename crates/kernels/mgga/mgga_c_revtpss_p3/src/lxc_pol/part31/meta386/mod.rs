//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta386<F: Float>(t3172: F, t4868: F, t1041: F, t3168: F, t4878: F, t11150: F, t3181: F, t11144: F, t11852: F, t3124: F, t4820: F, t1655: F, t697: F) -> (F, F, F, F, F, F, F) {
        let (t16163, t16165, t16190, t16199, t16208, t16218, t16219) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1424::<F>(t3172, t4868, t1041, t3168, t4878, t11150, t3181, t11144, t11852, t3124, t4820, t1655, t697);
    (t16163, t16165, t16190, t16199, t16208, t16218, t16219)
}
