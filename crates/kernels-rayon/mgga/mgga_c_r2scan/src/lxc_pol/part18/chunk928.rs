//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 928/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk928(t10740: f64, t254: f64, t120: f64, t2176: f64, t531: f64, t2222: f64, t2096: f64, t2101: f64, t2105: f64, t265: f64, t277: f64, t3332: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10741 = t254 * t10740;
    let t10742 = 0.15573871527278325618e-1_f64 * t10741;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    let t10748 = t120 * t2222;
    let t10757 = t2101 * t2096 * t265 * t2105;
    let t10758 = t254 * t10757;
    let t10759 = 0.59512461497092438715e-1_f64 * t10758;
    let t10760 = t3332 * t277;
    (t10742, t10743, t10744, t10748, t10757, t10759, t10760)
}
