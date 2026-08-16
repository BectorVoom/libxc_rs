//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 487/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk487<F: Float>(t45: F, t57: F, t261: F, t190: F, t2258: F, t706: F, t2251: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2410 = t261 * t261;
    let t2411 = F::cast_from(1.0_f64) / t2410;
    let t2414 = t190 * t2258;
    let t2416 = F::cast_from(4.0_f64) * t706 * t2414;
    let t2422 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t80 * t2251 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t2258);
    let t2428 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83 * t2251 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t2258);
    let t2430 = t2422 / F::cast_from(2.0_f64) + t2428 / F::cast_from(2.0_f64);
    (t2410, t2411, t2414, t2416, t2430)
}
