//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1043/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1043(t73926: f64, t73887: f64, t73891: f64, t73899: f64, t73906: f64, t73920: f64, t73924: f64, t73929: f64, t73931: f64, t76755: f64, t76757: f64, t76759: f64, t76764: f64, t76766: f64, t76768: f64, t76769: f64, t76771: f64) -> f64 {
    let t79999 = 0.29085809927086856922e-4_f64 * t73926;
    let t80002 = -t76755 + t73887 - 0.17519306092901367187e-6_f64 * t73891 + t76757 - t76759 + t76764 - 0.87596530464506835932e-6_f64 * t73899 - t76766 - 0.87596530464506835932e-6_f64 * t73906 + t76768 - t76769 - 0.35038612185802734374e-6_f64 * t73920 - t76771 - 0.81756761766873046868e-5_f64 * t73924 + t79999 - 0.17519306092901367186e-5_f64 * t73929 + 0.87596530464506835932e-6_f64 * t73931;
    t80002
}
