//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 871/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk871(t41579: f64, t41581: f64, t41585: f64, t41604: f64, t41613: f64, t41619: f64, t41654: f64, t41656: f64, t41667: f64, t41716: f64, t41722: f64, t41725: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43745 = 0.1489760996265424379e-3_f64 * t41579;
    let t43746 = 0.39726959900411316772e-4_f64 * t41581;
    let t43752 = 0.11918087970123395032e-3_f64 * t41585;
    let t43757 = 0.60975299583150056624e-3_f64 * t41604;
    let t43761 = 0.60975299583150056624e-3_f64 * t41613;
    let t43763 = 0.60975299583150056624e-3_f64 * t41619;
    let t43783 = 0.11918087970123395032e-3_f64 * t41654;
    let t43784 = 0.36366215538993788974e-1_f64 * t41656;
    let t43792 = 0.86737941314158990616e-4_f64 * t41667;
    let t43810 = 0.19158786722982093702e1_f64 * t41716;
    let t43812 = 0.3193131120497015617e0_f64 * t41722;
    let t43813 = 0.95793933614910468512e0_f64 * t41725;
    (t43745, t43746, t43752, t43757, t43761, t43763, t43783, t43784, t43792, t43810, t43812, t43813)
}
