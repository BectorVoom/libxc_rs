//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1049/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1049(t6355: f64, t7707: f64, t1550: f64, t41548: f64, t34975: f64, t34976: f64, t7455: f64, t8440: f64, t1978: f64, t7228: f64, t8511: f64, t236: f64, t495: f64) -> (f64, f64, f64, f64, f64) {
    let t41789 = t6355 * t7707;
    let t41790 = 0.15965655602485078085e0_f64 * t41789;
    let t41791 = t1550 * t41548;
    let t41792 = 0.15965655602485078085e0_f64 * t41791;
    let t41796 = t34975 * t34976 * t8440 * t7455;
    let t41799 = t8511 * t7228 * t1978;
    let t41800 = t236 * t495;
    (t41790, t41792, t41796, t41799, t41800)
}
