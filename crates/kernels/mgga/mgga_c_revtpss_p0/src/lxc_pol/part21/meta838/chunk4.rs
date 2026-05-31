//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3143/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3143<F: Float>(t1168: F, t12423: F, t12429: F, t12430: F, t12486: F, t12487: F, t12504: F, t12508: F, t12511: F, t16948: F, t16959: F, t17023: F, t17032: F, t17085: F, t17086: F, t1745: F, t1756: F, t1757: F, t3452: F, t3477: F, t3479: F, t45075: F, t45188: F, t45190: F, t45194: F, t5125: F, t5147: F, t56268: F, t56271: F, t56275: F, t56277: F) -> F {
    let t57943 = -F::cast_from(6.0_f64) * t17023 * t12504 + F::cast_from(0.96491876992155210402e2_f64) * t17032 * t12508 - F::cast_from(6.0_f64) * t45194 * t5125 + F::cast_from(0.96491876992155210402e2_f64) * t45075 * t5147 - F::cast_from(12.0_f64) * t12511 * t16948 + F::cast_from(0.19298375398431042081e3_f64) * t12423 * t16959 - F::cast_from(6.0_f64) * t3452 * t17086 * t1168 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t17085 * t3479 * t1168 - F::cast_from(0.14035736694323150897e2_f64) * t12486 * t1757 * t12487 - F::cast_from(24.0_f64) * t12429 * t1745 * t12430 + F::cast_from(0.91082604192152556044e5_f64) * t45188 * t1756 * t45190 * t12487 - t56268 - t56271 - t56275 + t56277;
    t57943
}
