//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1099/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1099<F: Float>(t5: F, t30551: F, t117: F, t118: F, t18245: F, t1911: F, t2014: F, t2056: F, t2093: F, t2108: F, t25082: F, t29506: F, t29508: F, t30138: F, t30209: F, t30218: F, t30315: F, t30511: F, t30513: F, t4248: F, t508: F, t5887: F, t651: F, t6934: F, t7359: F, t7732: F, t7898: F, t7978: F, t7984: F, t8075: F, t8079: F, t8109: F, t8111: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t30552 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t30551);
    let t30553 = t30552 * t117;
    let t30555 = -t118 * t30511 - F::cast_from(2.0_f64) * t18245 * t2056 + F::cast_from(2.0_f64) * t1911 * t8075 - F::cast_from(2.0_f64) * t2014 * t30218 + t2014 * t30315 - F::cast_from(2.0_f64) * t2056 * t29508 - F::cast_from(4.0_f64) * t2056 * t30138 + t2093 * t6934 + t2108 * t29506 - F::cast_from(6.0_f64) * t25082 * t30513 - F::cast_from(4.0_f64) * t30209 * t651 - t30553 * t508 - F::cast_from(4.0_f64) * t4248 * t7978 - F::cast_from(4.0_f64) * t4248 * t7984 - F::cast_from(4.0_f64) * t5887 * t7359 - F::cast_from(4.0_f64) * t7732 * t7978 + F::cast_from(6.0_f64) * t7898 * t8079 + F::cast_from(2.0_f64) * t7898 * t8109 - F::cast_from(2.0_f64) * t7898 * t8111;
    (t30552, t30553, t30555)
}
