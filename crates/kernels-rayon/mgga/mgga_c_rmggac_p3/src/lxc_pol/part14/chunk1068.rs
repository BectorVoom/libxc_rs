//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1068/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1068(t27075: f64, t739: f64, t7577: f64, t35523: f64, t9222: f64, t36733: f64, t8450: f64, t7478: f64, t1970: f64, t209: f64, t236: f64, t40433: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t42081 = t739 * t7577 * t27075;
    let t42083 = t9222 * t35523;
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42087 = 0.19863479950205658386e-4_f64 * t42086;
    let t42091 = t1970 * t7231 * t236 * t40433 * t209;
    (t42081, t42083, t42087, t42091)
}
