//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta516<F: Float>(t25410: F, t93169: F, t2438: F, t837: F, t786: F, t92889: F, t2434: F, t251: F, t25304: F, t25374: F, t68: F, t785: F) -> (F, F, F, F, F, F, F) {
        let (t93170, t93173, t93179, t93182, t93189, t93190, t93238) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1838::<F>(t25410, t93169, t2438, t837, t786, t92889, t2434, t251, t25304, t25374, t68, t785);
    (t93170, t93173, t93179, t93182, t93189, t93190, t93238)
}
