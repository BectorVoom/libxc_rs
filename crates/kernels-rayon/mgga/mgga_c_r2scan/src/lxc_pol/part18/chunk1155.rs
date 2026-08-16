//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1155/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1155(t322: f64, t42615: f64, t42646: f64, t42677: f64, t42709: f64, t42742: f64, t42774: f64, t42807: f64, t1065: f64, t3229: f64, t11002: f64, t3269: f64, t1039: f64, t11862: f64, t12580: f64, t374: f64, t42441: f64, t42443: f64, t42447: f64, t42450: f64, t42452: f64, t42457: f64, t42460: f64, t42462: f64, t42465: f64, t42467: f64, t42471: f64, t42475: f64, t860: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t42809 = piecewise5(t323, t42615, t331, t42646 + t42677 + t42709 + t42742, t42774 + t42807);
    let t42811 = t1065 * t3229;
    let t42812 = t11002 * t42811;
    let t42814 = 5.0_f64 / 16.0_f64 * t3269 * t42812;
    let t42815 = 2.0_f64 * t1039 * t11862 + t12580 * t860 + t374 * t42809 + t42441 + t42443 - t42447 + t42450 - t42452 - t42457 + t42460 + t42462 - t42465 - t42467 + t42471 - t42475 - t42814;
    (t42814, t42815)
}
