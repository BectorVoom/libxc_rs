//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3174/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3174(t56159: f64, t56163: f64, t56167: f64, t58029: f64, t58032: f64, t58035: f64, t58038: f64, t58041: f64, t58044: f64, t58046: f64, t58048: f64, t58051: f64) -> f64 {
    let t58504 = 0.53814000000000000001e1_f64 * t56159 + 0.59793333333333333334e0_f64 * t56163 + 0.71752e1_f64 * t56167 + 0.147882e1_f64 * t58029 + 0.10954222222222222222e0_f64 * t58032 - 0.49293999999999999999e0_f64 * t58035 + 0.427258125e1_f64 * t58038 - 0.230371875e0_f64 * t58041 - 0.28483875e1_f64 * t58044 - 0.28483875e1_f64 * t58046 - 0.9494625e0_f64 * t58048 + 0.46074375e0_f64 * t58051;
    t58504
}
