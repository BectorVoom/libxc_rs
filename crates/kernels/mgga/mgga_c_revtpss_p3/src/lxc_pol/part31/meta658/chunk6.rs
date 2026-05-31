//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2228/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2228<F: Float>(t101211: F, t101215: F, t101342: F, t18281: F, t1923: F, t1927: F, t19661: F, t19666: F, t19680: F, t25129: F, t25132: F, t28077: F, t28081: F, t28086: F, t28090: F, t28093: F, t28147: F, t28154: F, t29525: F, t29526: F, t29529: F, t5819: F, t5825: F, t6954: F, t6968: F, t6977: F, t72: F, t7702: F, t7719: F, t7720: F, t92600: F, t92605: F, t92612: F) -> F {
    let t108931 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t101211 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t28154 * t101215 - F::cast_from(10.0_f64) * t101342 * t28147 - t7702 * t28081 / F::cast_from(3.0_f64) - t28093 * t7720 / F::cast_from(3.0_f64) - t7702 * t28086 / F::cast_from(3.0_f64) - t7702 * t28090 / F::cast_from(3.0_f64) - t6954 * t29526 / F::cast_from(6.0_f64) - t1923 * (-F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t92600 * t5819 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t92605 * t19661 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t25132 * t19666 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t25129 * t5825 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t25132 * t19680 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6968 * t18281 + t92612) * t72 * t1927 / F::cast_from(6.0_f64) - t1923 * t29525 * t6977 / F::cast_from(6.0_f64) - t6954 * t29529 / F::cast_from(3.0_f64) - t1923 * t28077 * t7719 / F::cast_from(3.0_f64);
    t108931
}
