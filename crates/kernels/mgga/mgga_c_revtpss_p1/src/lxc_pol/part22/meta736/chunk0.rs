//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2796/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2796<F: Float>(t10696: F, t72: F, t245: F, t10729: F, t9775: F, t10705: F, t10716: F, t10697: F, t136: F, t2452: F, t9720: F, t225: F) -> (F, F, F, F, F, F, F) {
    let t40672 = t10696 * t72;
    let t40673 = t40672 * t245;
    let t40679 = t9775 * t10729;
    let t40681 = t10716 * t10705;
    let t40683 = t10697 * t136;
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    (t40672, t40673, t40679, t40681, t40683, t40688, t40689)
}
