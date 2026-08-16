//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 666/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk666(t4823: f64, t4825: f64, t4741: f64, t4744: f64, t4746: f64, t4748: f64, t4751: f64, t4733: f64, t4736: f64, t4739: f64, t401: f64, t384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4826 = t4823 * t4825;
    let t4827 = 0.96491876992155210402e2_f64 * t4826;
    let t4831 = 0.93011851851851851854e0_f64 * t4741;
    let t4832 = 0.13651666666666666667e0_f64 * t4744;
    let t4833 = 0.27303333333333333333e0_f64 * t4746;
    let t4834 = 0.3185388888888888889e0_f64 * t4748;
    let t4835 = 0.36514074074074074075e0_f64 * t4751;
    let t4836 = -0.25319e1_f64 * t4733 + 0.16879333333333333333e1_f64 * t4736 - 0.19692555555555555555e1_f64 * t4739 - t4831 + t4832 - t4833 - t4834 - t4835;
    let t4837 = t4836 * t401;
    let t4838 = t384 * t4837;
    (t4827, t4831, t4832, t4833, t4834, t4835, t4838)
}
