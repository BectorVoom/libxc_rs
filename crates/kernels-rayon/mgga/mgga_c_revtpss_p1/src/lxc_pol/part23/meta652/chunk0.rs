//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2380/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2380(t245: f64, t40672: f64, t10697: f64, t136: f64, t2452: f64, t9720: f64, t225: f64, t268: f64, t2665: f64, t10868: f64, t240: f64, t2237: f64, t2482: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40673 = t40672 * t245;
    let t40683 = t10697 * t136;
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    let t40690 = t268 * t40689;
    let t40691 = t40690 * t2665;
    let t40693 = t10868 * t240;
    let t40710 = t2482 * t849 * t2237;
    (t40673, t40683, t40688, t40689, t40690, t40691, t40693, t40710)
}
