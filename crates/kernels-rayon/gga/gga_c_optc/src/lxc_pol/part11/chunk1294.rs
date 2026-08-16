//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1294/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1294(t57086: f64, t57098: f64, t787: f64, t13939: f64, t4793: f64, t16729: f64, t3665: f64, t780: f64, t39565: f64, t49404: f64, t49406: f64, t57057: f64, t57060: f64, t57063: f64, t57066: f64, t57069: f64, t57071: f64, t57073: f64) -> (f64, f64, f64, f64, f64) {
    let t57099 = t57086 + t57098;
    let t57100 = t787 * t57099;
    let t57102 = t13939 * t4793;
    let t57104 = t3665 * t16729;
    let t57106 = t780 * t57099;
    let t57108 = 0.10954222222222222222e1_f64 * t39565 + 0.13145066666666666666e1_f64 * t49404 - 0.43816888888888888888e0_f64 * t49406 - 0.29896666666666666667e0_f64 * t57057 + 0.71752e1_f64 * t57060 + 0.17938e1_f64 * t57063 + 0.46074375e0_f64 * t57066 + 0.1151859375e0_f64 * t57069 - 0.28483875e1_f64 * t57071 - 0.3560484375e1_f64 * t57073 + 0.3071625e0_f64 * t57100 + 0.85451625e1_f64 * t57102 - 0.379785e1_f64 * t57104 + 0.1898925e1_f64 * t57106;
    (t57100, t57102, t57104, t57106, t57108)
}
