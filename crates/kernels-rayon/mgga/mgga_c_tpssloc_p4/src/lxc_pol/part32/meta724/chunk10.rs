//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2328/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328(t104280: f64, t2132: f64, t24746: f64, t1714: f64, t18221: f64, t18225: f64, t18237: f64, t18940: f64, t2121: f64, t2136: f64, t24650: f64, t29562: f64, t29594: f64, t3448: f64, t475: f64, t6729: f64, t68: f64, t7321: f64, t7326: f64, t7328: f64, t7573: f64, t95340: f64, t95346: f64, t95387: f64, t95515: f64, t95517: f64, t95520: f64) -> f64 {
    let t104337 = t2132 * t104280 * t24746;
    let t104351 = -t95515 - t2121 * t3448 * t18237 / 144.0_f64 - t2121 * t3448 * t18225 / 72.0_f64 - t2121 * t3448 * t18221 / 48.0_f64 + 0.10093189023535097714e-3_f64 * t7326 * t7328 * t18940 * t68 * t475 - 0.10093189023535097714e-3_f64 * t24650 * t29594 - 0.10093189023535097714e-3_f64 * t104337 - 0.72670960969452703541e-2_f64 * t29562 * t6729 * t2136 + t95517 + t95520 / 648.0_f64 - 0.40372756094140390856e-3_f64 * t95387 * t95340 + 0.20186378047070195428e-3_f64 * t95387 * t95346 + 0.20186378047070195428e-3_f64 * t2132 * t7573 * t1714 * t7321;
    t104351
}
