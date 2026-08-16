//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3539/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3539(t1028: f64, t11774: f64, t13396: f64, t16049: f64, t19718: f64, t20039: f64, t3092: f64, t3205: f64, t371: f64, t372: f64, t373: f64, t42265: f64, t4573: f64, t53545: f64, t55002: f64, t55004: f64, t55011: f64, t6339: f64, t65122: f64, t67186: f64, t67195: f64, t67199: f64, t67206: f64, t67213: f64, t67215: f64) -> f64 {
    let t67218 = -0.20325460441158986416e-2_f64 * t55002 + 0.47637797908966374413e-4_f64 * t67186 - 0.57165357490759649296e-3_f64 * t55004 + 0.45732285992607719436e-2_f64 * t16049 * t19718 + 0.34299214494455789578e-2_f64 * t55011 * t3092 * t4573 * t13396 + 0.95275595817932748827e-4_f64 * t67195 - 0.45732285992607719436e-2_f64 * t42265 * t6339 + 0.57165357490759649296e-3_f64 * t67199 - 0.57165357490759649296e-3_f64 * t11774 * t53545 * t20039 - 0.95275595817932748827e-4_f64 * t67206 + 0.85748036236139473944e-3_f64 * t3205 * t371 * t372 * t373 * t65122 - 0.57165357490759649296e-3_f64 * t67213 + 0.22866142996303859718e-2_f64 * t67215 * t1028;
    t67218
}
