//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1003/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1003(t121660: f64, t126409: f64, t126412: f64, t126417: f64, t126418: f64, t126419: f64, t126422: f64, t126423: f64, t2054: f64, t25168: f64, t26700: f64, t26713: f64, t26728: f64, t2718: f64, t28311: f64, t28316: f64, t31423: f64, t33405: f64, t5637: f64, t5658: f64, t7087: f64, t7517: f64, t7537: f64, t7538: f64, t7841: f64, t855: f64, t98166: f64, t98279: f64) -> f64 {
    let t127947 = 4.0_f64 * t26700 * t7517 + 4.0_f64 * t855 * t2718 * t7841 * t7537 - t31423 * t5658 + 0.38381794893125283518e-1_f64 * t121660 - t126409 - t126412 - t126417 + t126418 - t126419 + 2.0_f64 * t31423 * t5637 + t126422 - t98166 * t2054 - 6.0_f64 * t7087 * t28311 - 2.0_f64 * t26713 * t7538 - 12.0_f64 * t98279 * t33405 + t126423 - 6.0_f64 * t25168 * t26728 * t28316;
    t127947
}
