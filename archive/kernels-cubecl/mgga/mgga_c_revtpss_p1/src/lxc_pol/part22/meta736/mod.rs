//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta736 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2796;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta736<F: Float>(t10696: F, t72: F, t245: F, t10729: F, t9775: F, t10705: F, t10716: F, t10697: F, t136: F, t2452: F, t9720: F, t225: F) -> (F, F, F, F, F, F, F) {
        let (t40672, t40673, t40679, t40681, t40683, t40688, t40689) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2796::<F>(t10696, t72, t245, t10729, t9775, t10705, t10716, t10697, t136, t2452, t9720, t225);
    (t40672, t40673, t40679, t40681, t40683, t40688, t40689)
}
