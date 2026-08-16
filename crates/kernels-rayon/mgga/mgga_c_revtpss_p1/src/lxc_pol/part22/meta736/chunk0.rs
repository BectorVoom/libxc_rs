//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2796/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2796(t10696: f64, t72: f64, t245: f64, t10729: f64, t9775: f64, t10705: f64, t10716: f64, t10697: f64, t136: f64, t2452: f64, t9720: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40672 = t10696 * t72;
    let t40673 = t40672 * t245;
    let t40679 = t9775 * t10729;
    let t40681 = t10716 * t10705;
    let t40683 = t10697 * t136;
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    (t40672, t40673, t40679, t40681, t40683, t40688, t40689)
}
