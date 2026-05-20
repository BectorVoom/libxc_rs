//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta315<F: Float>(t136: F, t860: F, t2457: F, t2710: F, t10519: F, t10524: F, t10533: F, t10539: F, t10543: F, t10548: F, t10639: F, t10645: F, t10647: F, t10651: F, t10655: F, t10657: F, t10661: F, t10666: F, t10910: F, t213: F, t234: F, t2646: F, t2724: F, t2815: F, t820: F, t837: F, t879: F) -> (F, F, F) {
        let (t10914, t10916, t10918) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1588::<F>(t136, t860, t2457, t2710, t10519, t10524, t10533, t10539, t10543, t10548, t10639, t10645, t10647, t10651, t10655, t10657, t10661, t10666, t10910, t213, t234, t2646, t2724, t2815, t820, t837, t879);
    (t10914, t10916, t10918)
}
