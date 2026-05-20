//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3539/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3539<F: Float>(t1028: F, t11774: F, t13396: F, t16049: F, t19718: F, t20039: F, t3092: F, t3205: F, t371: F, t372: F, t373: F, t42265: F, t4573: F, t53545: F, t55002: F, t55004: F, t55011: F, t6339: F, t65122: F, t67186: F, t67195: F, t67199: F, t67206: F, t67213: F, t67215: F) -> F {
    let t67218 = -F::cast_from(0.20325460441158986416e-2_f64) * t55002 + F::cast_from(0.47637797908966374413e-4_f64) * t67186 - F::cast_from(0.57165357490759649296e-3_f64) * t55004 + F::cast_from(0.45732285992607719436e-2_f64) * t16049 * t19718 + F::cast_from(0.34299214494455789578e-2_f64) * t55011 * t3092 * t4573 * t13396 + F::cast_from(0.95275595817932748827e-4_f64) * t67195 - F::cast_from(0.45732285992607719436e-2_f64) * t42265 * t6339 + F::cast_from(0.57165357490759649296e-3_f64) * t67199 - F::cast_from(0.57165357490759649296e-3_f64) * t11774 * t53545 * t20039 - F::cast_from(0.95275595817932748827e-4_f64) * t67206 + F::cast_from(0.85748036236139473944e-3_f64) * t3205 * t371 * t372 * t373 * t65122 - F::cast_from(0.57165357490759649296e-3_f64) * t67213 + F::cast_from(0.22866142996303859718e-2_f64) * t67215 * t1028;
    t67218
}
