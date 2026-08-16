//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3265/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3265(t73481: f64, t73493: f64, t73515: f64, t74106: f64, t48280: f64, t48282: f64, t48285: f64, t74111: f64, t48287: f64, t47067: f64, t47070: f64, t47072: f64, t47076: f64, t48279: f64, t48291: f64, t48293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85918 = 0.17544670867903938621e1_f64 * t73481;
    let t85919 = 0.54934341918019635162e-3_f64 * t73493;
    let t85920 = 0.73245789224026180216e-3_f64 * t73515;
    let t85921 = 0.51947577317044391276e2_f64 * t74106;
    let t85922 = 0.17090684152272775384e-2_f64 * t48280;
    let t85923 = 0.10526802520742363173e2_f64 * t48282;
    let t85924 = 0.10526802520742363173e2_f64 * t48285;
    let t85925 = 12.0_f64 * t74111;
    let t85926 = 72.0_f64 * t48287;
    let t85927 = -t85918 - t85919 + t85920 + t47067 - t85921 - t47070 - t47072 - t48279 - t85922 - t85923 - t47076 + t85924 - t85925 + t85926 + t48291 - t48293;
    (t85918, t85919, t85920, t85921, t85922, t85923, t85924, t85925, t85926, t85927)
}
