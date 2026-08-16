//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2171/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2171(t87583: f64, t234: f64, t4265: f64, t6552: f64, t6637: f64, t776: f64, t23110: f64, t23185: f64, t25237: f64, t23168: f64, t25307: f64, t13263: f64, t13397: f64, t25261: f64, t2633: f64, t2679: f64, t4182: f64, t4281: f64, t4291: f64, t81656: f64, t81670: f64, t81689: f64, t81691: f64, t829: f64, t87566: f64, t87567: f64, t87575: f64, t87578: f64, t87582: f64) -> f64 {
    let t87584 = 0.76763589786250567036e-1_f64 * t87583;
    let t87586 = t234 * t4265;
    let t87589 = t6552 * t6637 * t87586 * t776;
    let t87601 = t23185 * t23110 * t25237;
    let t87602 = 0.82246703342411321824e-2_f64 * t87601;
    let t87603 = t23168 * t25307;
    let t87604 = 0.76763589786250567036e-1_f64 * t87603;
    let t87606 = -t87566 + 4.0_f64 * t4281 * t87567 * t4182 - t4291 * t25261 * t2679 + 0.16449340668482264365e-1_f64 * t81656 - 0.16449340668482264365e-1_f64 * t87575 - 0.82246703342411321825e-2_f64 * t87578 + t87582 - t87584 + 0.82246703342411321824e-2_f64 * t81670 - 0.3289868133696452873e-1_f64 * t87589 - 6.0_f64 * t13397 * t25261 * t13263 + 6.0_f64 * t4281 * t25261 * t2633 - 2.0_f64 * t4291 * t87567 * t829 + t87602 + t87604 - t81689 + 0.41123351671205660912e-2_f64 * t81691;
    t87606
}
