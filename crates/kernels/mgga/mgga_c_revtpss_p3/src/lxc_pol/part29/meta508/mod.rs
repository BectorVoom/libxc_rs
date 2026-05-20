//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta508<F: Float>(t1353: F, t5778: F, t1448: F, t1501: F, t2371: F, t4245: F, t670: F, t10301: F, t607: F, t1927: F, t2248: F, t644: F, t6977: F) -> (F, F, F, F, F, F, F) {
        let (t75353, t75365, t75485, t75667, t92565, t92569, t92576) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1826::<F>(t1353, t5778, t1448, t1501, t2371, t4245, t670, t10301, t607, t1927, t2248, t644, t6977);
    (t75353, t75365, t75485, t75667, t92565, t92569, t92576)
}
