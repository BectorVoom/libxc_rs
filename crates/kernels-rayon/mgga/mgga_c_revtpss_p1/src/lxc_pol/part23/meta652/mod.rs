//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta652(t245: f64, t40672: f64, t10697: f64, t136: f64, t2452: f64, t9720: f64, t225: f64, t268: f64, t2665: f64, t10868: f64, t240: f64, t2237: f64, t2482: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40673, t40683, t40688, t40689, t40690, t40691, t40693, t40710) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2380(t245, t40672, t10697, t136, t2452, t9720, t225, t268, t2665, t10868, t240, t2237, t2482, t849);
    (t40673, t40683, t40688, t40689, t40690, t40691, t40693, t40710)
}
