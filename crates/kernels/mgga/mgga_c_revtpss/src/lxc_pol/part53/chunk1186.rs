//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1186/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1186<F: Float>(t31767: F, t31772: F, t4364: F, t4533: F, t119941: F, t120067: F, t120071: F, t120074: F, t120088: F, t120091: F, t120107: F, t120112: F, t120115: F, t120118: F, t120119: F, t120133: F, t126340: F, t126345: F, t126358: F, t27267: F, t27317: F, t31787: F, t31812: F, t31824: F, t32426: F, t33704: F, t33707: F, t34075: F, t8649: F, t886: F) -> F {
    let t126365 = t31767 * t4364 * t31772 * t4533;
    let t126367 = F::cast_from(0.11423947533020470523e1_f64) * t34075 * t31824 + F::cast_from(0.28234466758480466999e-3_f64) * t126340 + t120067 + F::cast_from(0.3718732920905101082e-3_f64) * t126345 + t120071 - F::cast_from(0.34271842599061411569e1_f64) * t8649 * t31812 * t33707 * t886 - F::cast_from(0.11423947533020470523e1_f64) * t32426 * t33704 - F::cast_from(0.17347256376410398924e1_f64) * t31787 * t27267 - t120074 + F::cast_from(0.34694512752820797848e1_f64) * t119941 * t27317 - F::cast_from(0.1859366460452550541e-3_f64) * t126358 - t120088 - F::cast_from(0.14456046980341999104e-1_f64) * t120091 + F::cast_from(0.66119071333692697238e-4_f64) * t120107 - t120112 + t120115 - t120118 - F::cast_from(0.3718732920905101082e-4_f64) * t120119 - t120133 - F::cast_from(0.28234466758480466999e-3_f64) * t126365;
    t126367
}
