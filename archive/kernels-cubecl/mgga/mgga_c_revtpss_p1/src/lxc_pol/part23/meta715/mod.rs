//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2474;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta715<F: Float>(t48255: F, t47007: F, t13597: F, t2626: F, t5571: F, t9387: F, t47013: F, t13613: F, t2619: F, t9323: F, t47019: F, t47073: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48256, t48259, t48261, t48262, t48266, t48268, t48269, t48271, t48279) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2474::<F>(t48255, t47007, t13597, t2626, t5571, t9387, t47013, t13613, t2619, t9323, t47019, t47073);
    (t48256, t48259, t48261, t48262, t48266, t48268, t48269, t48271, t48279)
}
