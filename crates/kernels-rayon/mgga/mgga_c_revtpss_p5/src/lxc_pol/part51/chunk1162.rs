//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1162/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1162(t31949: f64, t33800: f64, t1668: f64, t8507: f64, t73: f64, t27619: f64, t31902: f64, t1043: f64, t1089: f64, t120362: f64, t120368: f64, t120370: f64, t120374: f64, t120385: f64, t120425: f64, t120532: f64, t120625: f64, t120654: f64, t247: f64, t27664: f64, t3116: f64, t31905: f64, t31920: f64, t31953: f64, t33791: f64, t33804: f64, t385: f64, t4763: f64, t4772: f64, t7160: f64, t99638: f64) -> (f64, f64, f64) {
    let t126651 = t33800 * t31949;
    let t126659 = t8507 * t1668;
    let t126660 = t126659 * t73;
    let t126667 = t31902 * t27619;
    let t126673 = -0.56468933516960933998e-3_f64 * t31920 * t247 * t3116 * t385 * t4772 - 0.17135921299530705785e1_f64 * t120425 * t33804 - 0.37187329209051010821e-3_f64 * t120368 + 0.37187329209051010821e-3_f64 * t120370 + 0.24791552806034007214e-3_f64 * t120374 + 0.3718732920905101082e-3_f64 * t126651 * t31953 + 0.3427184259906141157e1_f64 * t120625 * t33791 * t1043 * t1089 - 0.18822977838986977999e-3_f64 * t120385 - 0.34271842599061411569e1_f64 * t120654 * t126660 * t27664 - 0.34694512752820797848e1_f64 * t120362 * t7160 * t4763 - 0.17135921299530705785e1_f64 * t126667 * t31905 + 0.34694512752820797848e1_f64 * t120532 * t7160 * t99638;
    (t126659, t126660, t126673)
}
