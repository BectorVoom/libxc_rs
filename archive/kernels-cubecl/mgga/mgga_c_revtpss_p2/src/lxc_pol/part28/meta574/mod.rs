//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta574<F: Float>(t11970: F, t1973: F, t1058: F, t25554: F, t3201: F, t7126: F, t25561: F, t7114: F, t25566: F, t1024: F, t25576: F, t25525: F, t3123: F) -> (F, F, F, F, F, F, F, F) {
        let (t93611, t93616, t93618, t93620, t93622, t93627, t93646, t93649) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2037::<F>(t11970, t1973, t1058, t25554, t3201, t7126, t25561, t7114, t25566, t1024, t25576, t25525, t3123);
    (t93611, t93616, t93618, t93620, t93622, t93627, t93646, t93649)
}
