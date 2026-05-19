//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1162/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1162<F: Float>(t31949: F, t33800: F, t1668: F, t8507: F, t73: F, t27619: F, t31902: F, t1043: F, t1089: F, t120362: F, t120368: F, t120370: F, t120374: F, t120385: F, t120425: F, t120532: F, t120625: F, t120654: F, t247: F, t27664: F, t3116: F, t31905: F, t31920: F, t31953: F, t33791: F, t33804: F, t385: F, t4763: F, t4772: F, t7160: F, t99638: F) -> (F, F, F) {
    let t126651 = t33800 * t31949;
    let t126659 = t8507 * t1668;
    let t126660 = t126659 * t73;
    let t126667 = t31902 * t27619;
    let t126673 = -F::cast_from(0.56468933516960933998e-3_f64) * t31920 * t247 * t3116 * t385 * t4772 - F::cast_from(0.17135921299530705785e1_f64) * t120425 * t33804 - F::cast_from(0.37187329209051010821e-3_f64) * t120368 + F::cast_from(0.37187329209051010821e-3_f64) * t120370 + F::cast_from(0.24791552806034007214e-3_f64) * t120374 + F::cast_from(0.3718732920905101082e-3_f64) * t126651 * t31953 + F::cast_from(0.3427184259906141157e1_f64) * t120625 * t33791 * t1043 * t1089 - F::cast_from(0.18822977838986977999e-3_f64) * t120385 - F::cast_from(0.34271842599061411569e1_f64) * t120654 * t126660 * t27664 - F::cast_from(0.34694512752820797848e1_f64) * t120362 * t7160 * t4763 - F::cast_from(0.17135921299530705785e1_f64) * t126667 * t31905 + F::cast_from(0.34694512752820797848e1_f64) * t120532 * t7160 * t99638;
    (t126659, t126660, t126673)
}
