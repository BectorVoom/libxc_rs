//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2160/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2160(t23098: f64, t7496: f64, t87447: f64, t6590: f64, t6646: f64, t25130: f64, t81918: f64, t81921: f64, t81924: f64, t81926: f64, t81936: f64, t87418: f64, t87422: f64, t87426: f64, t87428: f64, t87430: f64, t87432: f64, t87437: f64, t87438: f64, t87440: f64, t87444: f64, t87445: f64) -> f64 {
    let t87449 = t87447 * t7496 * t23098;
    let t87451 = t6590 * t6646;
    let t87453 = t87451 * t25130 * t23098;
    let t87455 = 0.16956557559538964158e-1_f64 * t87418 - t87422 / 4.0_f64 - t87426 + 0.84782787797694820792e-2_f64 * t87428 - t87430 / 48.0_f64 - 0.11304371706359309439e-1_f64 * t87432 - 0.6728792682356731809e-4_f64 * t81918 - t81921 + 0.33643963411783659045e-4_f64 * t81924 - 7.0_f64 / 2304.0_f64 * t81926 + t87437 - t87438 + 0.84782787797694820794e-2_f64 * t81936 - t87440 + t87444 + 0.10093189023535097714e-3_f64 * t87445 - 0.16956557559538964158e-1_f64 * t87449 + 0.24223653656484234512e-2_f64 * t87453;
    t87455
}
