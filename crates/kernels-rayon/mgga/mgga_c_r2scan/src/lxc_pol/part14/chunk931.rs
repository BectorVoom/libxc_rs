//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 931/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk931(t10740: f64, t254: f64, t120: f64, t2176: f64, t531: f64, t2233: f64, t3290: f64, t2222: f64, t2225: f64, t2186: f64, t261: f64, t7628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10741 = t254 * t10740;
    let t10743 = t120 * t2176;
    let t10744 = t10743 * t531;
    let t10745 = 0.25610080155860322884e0_f64 * t10744;
    let t10746 = t3290 * t2233;
    let t10748 = t120 * t2222;
    let t10749 = t10748 * t2225;
    let t10752 = t261 * t2186;
    let t10753 = t7628 * t10752;
    (t10741, t10743, t10744, t10745, t10746, t10749, t10752, t10753)
}
