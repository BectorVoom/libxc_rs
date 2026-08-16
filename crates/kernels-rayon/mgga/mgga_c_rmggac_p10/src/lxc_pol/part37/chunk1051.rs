//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1051/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1051(t70905: f64, t74228: f64, t74235: f64, t74238: f64, t74243: f64, t76931: f64, t76932: f64, t76935: f64, t76937: f64, t76939: f64, t76940: f64, t76941: f64, t76942: f64, t76943: f64, t76946: f64, t76947: f64, t76948: f64) -> f64 {
    let t80071 = 0.70077224371605468748e-6_f64 * t74228 + t76931 - t76932 + 0.35038612185802734374e-6_f64 * t74235 + t76935 - 0.52557918278704101561e-6_f64 * t74238 + t76937 + 0.76860658247009135562e-5_f64 * t74243 - t76939 - t76940 + t76941 + t76942 - t70905 - t76943 - t76946 - t76947 - t76948;
    t80071
}
