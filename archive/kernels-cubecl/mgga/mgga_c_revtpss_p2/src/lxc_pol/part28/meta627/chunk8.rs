//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2255/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2255<F: Float>(t13388: F, t76: F, t13312: F, t13392: F, t13396: F, t1469: F, t15936: F, t1923: F, t1926: F, t1927: F, t25129: F, t25132: F, t25139: F, t25146: F, t25150: F, t28077: F, t28078: F, t28081: F, t28086: F, t28089: F, t28090: F, t4181: F, t4186: F, t6954: F, t6963: F, t6968: F, t6973: F, t6977: F, t72: F, t7715: F, t7719: F, t7720: F, t92597: F, t92600: F, t92605: F, t92612: F) -> F {
    let t101303 = t76 * t13388;
    let t101309 = -t6954 * t28078 / F::cast_from(3.0_f64) - t6954 * t28081 / F::cast_from(3.0_f64) - t1923 * (F::cast_from(220.0_f64) / F::cast_from(27.0_f64) * t92597 * t1469 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t92600 * t4181 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t25129 * t4186 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t92605 * t15936 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t25132 * t13396 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t25132 * t13392 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6968 * t13312 + t92612) * t72 * t1927 / F::cast_from(6.0_f64) - t1923 * t28077 * t6977 / F::cast_from(3.0_f64) - t1923 * t7715 * t25146 / F::cast_from(6.0_f64) - t25150 * t7720 / F::cast_from(6.0_f64) - t6954 * t28086 / F::cast_from(3.0_f64) - t6954 * t28090 / F::cast_from(3.0_f64) - t1923 * t25139 * t7719 / F::cast_from(6.0_f64) - t1923 * t6973 * t28089 / F::cast_from(3.0_f64) - t1923 * t1926 * t101303 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6963 * t28078;
    t101309
}
