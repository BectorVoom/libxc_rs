//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 966/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk966(t114932: f64, t112915: f64, t112920: f64, t112927: f64, t112932: f64, t112936: f64, t112942: f64, t114913: f64, t114916: f64, t114926: f64, t2053: f64, t22978: f64, t23190: f64, t23278: f64, t23281: f64, t24305: f64, t25168: f64, t26728: f64, t2713: f64, t2718: f64, t31400: f64, t6632: f64, t7092: f64, t7107: f64, t855: f64) -> f64 {
    let t114933 = 0.82246703342411321824e-2_f64 * t114932;
    let t114934 = 2.0_f64 * t855 * t2718 * t2053 * t23190 + 4.0_f64 * t23281 * t7092 - t112915 + 4.0_f64 * t24305 * t6632 - t112920 + 0.49348022005446793095e-1_f64 * t114913 + t112927 - t112932 + 0.16449340668482264365e-1_f64 * t114916 - 2.0_f64 * t23281 * t7107 - 12.0_f64 * t25168 * t26728 * t22978 - 2.0_f64 * t2713 * t31400 - 0.16449340668482264365e-1_f64 * t114926 + t112936 + 4.0_f64 * t23278 * t7092 - t114933 - t112942;
    t114934
}
