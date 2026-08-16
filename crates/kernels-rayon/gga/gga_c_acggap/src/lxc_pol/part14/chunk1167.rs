//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1167/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1167(t1988: f64, t9687: f64, t2001: f64, t6361: f64, t31471: f64, t31473: f64, t31477: f64, t31479: f64, t35624: f64, t35632: f64, t35636: f64, t35647: f64, t37645: f64, t37652: f64, t37653: f64, t40083: f64, t40086: f64, t40089: f64, t40092: f64, t40095: f64) -> f64 {
    let t40099 = t1988 * t9687;
    let t40101 = t2001 * t6361;
    let t40103 = t35624 - t35632 + t35636 - 7.0_f64 / 72.0_f64 * t40083 - 0.22921875e-1_f64 * t40086 - 0.4584375e-1_f64 * t40089 - 0.21437009059034868486e-2_f64 * t40092 + 0.10718504529517434243e-2_f64 * t40095 - t31471 + t31473 + t37645 - t35647 - 0.65369791666666666667e-1_f64 * t31477 + 0.66040993808168719343e-2_f64 * t31479 + 0.10718504529517434243e-3_f64 * t40099 - 0.51448821741683684367e-1_f64 * t40101 - t37652 - t37653;
    t40103
}
