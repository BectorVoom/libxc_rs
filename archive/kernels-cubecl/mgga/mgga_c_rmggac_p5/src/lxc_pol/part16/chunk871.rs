//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 871/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk871<F: Float>(t41579: F, t41581: F, t41585: F, t41604: F, t41613: F, t41619: F, t41654: F, t41656: F, t41667: F, t41716: F, t41722: F, t41725: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43745 = F::cast_from(0.1489760996265424379e-3_f64) * t41579;
    let t43746 = F::cast_from(0.39726959900411316772e-4_f64) * t41581;
    let t43752 = F::cast_from(0.11918087970123395032e-3_f64) * t41585;
    let t43757 = F::cast_from(0.60975299583150056624e-3_f64) * t41604;
    let t43761 = F::cast_from(0.60975299583150056624e-3_f64) * t41613;
    let t43763 = F::cast_from(0.60975299583150056624e-3_f64) * t41619;
    let t43783 = F::cast_from(0.11918087970123395032e-3_f64) * t41654;
    let t43784 = F::cast_from(0.36366215538993788974e-1_f64) * t41656;
    let t43792 = F::cast_from(0.86737941314158990616e-4_f64) * t41667;
    let t43810 = F::cast_from(0.19158786722982093702e1_f64) * t41716;
    let t43812 = F::cast_from(0.3193131120497015617e0_f64) * t41722;
    let t43813 = F::cast_from(0.95793933614910468512e0_f64) * t41725;
    (t43745, t43746, t43752, t43757, t43761, t43763, t43783, t43784, t43792, t43810, t43812, t43813)
}
