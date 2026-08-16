//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1380/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1380(t22986: f64, t23270: f64, t31332: f64, t98960: f64, t114770: f64, t25054: f64, t25038: f64, t25040: f64, t114606: f64, t118488: f64, t118491: f64, t118498: f64, t118499: f64, t24297: f64, t26690: f64, t2718: f64, t31311: f64, t31400: f64, t4147: f64, t4268: f64, t4300: f64, t6627: f64, t7517: f64, t855: f64, t8562: f64) -> f64 {
    let t121326 = t22986 * t23270 * t31332 * t98960;
    let t121336 = t22986 * t114770 * t25054;
    let t121339 = t25038 * t114770 * t25040;
    let t121343 = -t118488 - t4268 * t31400 + t118491 + 2.0_f64 * t6627 * t26690 + t118498 + t118499 - 0.3289868133696452873e-1_f64 * t121326 - 0.38381794893125283518e-1_f64 * t114606 + 2.0_f64 * t855 * t2718 * t8562 * t4300 + 2.0_f64 * t24297 * t7517 + 0.16449340668482264365e-1_f64 * t121336 + 0.49348022005446793095e-1_f64 * t121339 + 2.0_f64 * t4147 * t31311;
    t121343
}
