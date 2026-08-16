//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1121/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1121(t76545: f64, t15887: f64, t302: f64, t72: f64, t72170: f64, t72178: f64, t72193: f64, t73624: f64, t78591: f64, t78592: f64, t78593: f64, t78595: f64, t78597: f64, t78602: f64, t78605: f64, t78609: f64, t78611: f64, t78612: f64, t78613: f64) -> f64 {
    let t80537 = 0.40992351065071538966e-4_f64 * t76545;
    let t80538 = t15887 * t302 * t72 - t72170 + t72178 + t72193 - t73624 + t78591 + t78592 - t78593 + t78595 - t78597 - t78602 - t78605 + t78609 - t78611 + t78612 + t78613 - t80537;
    t80538
}
