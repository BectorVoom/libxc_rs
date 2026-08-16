//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1151/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1151(t31767: f64, t31772: f64, t4364: f64, t4533: f64, t119941: f64, t120067: f64, t120071: f64, t120074: f64, t120088: f64, t120091: f64, t120107: f64, t120112: f64, t120115: f64, t120118: f64, t120119: f64, t120133: f64, t126340: f64, t126345: f64, t126358: f64, t27267: f64, t27317: f64, t31787: f64, t31812: f64, t31824: f64, t32426: f64, t33704: f64, t33707: f64, t34075: f64, t8649: f64, t886: f64) -> f64 {
    let t126365 = t31767 * t4364 * t31772 * t4533;
    let t126367 = 0.11423947533020470523e1_f64 * t34075 * t31824 + 0.28234466758480466999e-3_f64 * t126340 + t120067 + 0.3718732920905101082e-3_f64 * t126345 + t120071 - 0.34271842599061411569e1_f64 * t8649 * t31812 * t33707 * t886 - 0.11423947533020470523e1_f64 * t32426 * t33704 - 0.17347256376410398924e1_f64 * t31787 * t27267 - t120074 + 0.34694512752820797848e1_f64 * t119941 * t27317 - 0.1859366460452550541e-3_f64 * t126358 - t120088 - 0.14456046980341999104e-1_f64 * t120091 + 0.66119071333692697238e-4_f64 * t120107 - t120112 + t120115 - t120118 - 0.3718732920905101082e-4_f64 * t120119 - t120133 - 0.28234466758480466999e-3_f64 * t126365;
    t126367
}
