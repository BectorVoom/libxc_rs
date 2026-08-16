//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 644/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk644(t4753: f64, t9252: f64, t16745: f64, t16748: f64, t16751: f64, t2: f64, t4668: f64, t1775: f64, t4765: f64, t4768: f64, t4759: f64, t458: f64, t4776: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17239 = t9252 * t4753;
    let t17249 = t16745 / 9.0_f64;
    let t17250 = 2.0_f64 / 9.0_f64 * t16748;
    let t17251 = 2.0_f64 / 27.0_f64 * t16751;
    let t17259 = t2 * t4668;
    let t17272 = t1775 * t4765;
    let t17274 = t1775 * t4768;
    let t17276 = t1775 * t4759;
    let t17279 = t458 * t4776;
    (t17239, t17249, t17250, t17251, t17259, t17272, t17274, t17276, t17279)
}
