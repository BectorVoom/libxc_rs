//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 638/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk638<F: Float>(t11986: F, t447: F, t1064: F, t3701: F, t448: F, t3691: F, t535: F, t10115: F, t10118: F, t10131: F, t10134: F, t10137: F, t10139: F, t1063: F, t11978: F, t11983: F, t2268: F, t9072: F, t9077: F, t9085: F) -> (F, F) {
    let t11987 = t11986 * t447;
    let t11988 = t1064 * t11987;
    let t11991 = t3701 * t448;
    let t11994 = t535 * t3691;
    let t11997 = -F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t11978 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t11983 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t11988 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t11991 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t11994 - t9072 + t9077 + t9085 + t10115 + t10118 - t10131 - t10134 - t10137 + t10139;
    (t11987, t11997)
}
