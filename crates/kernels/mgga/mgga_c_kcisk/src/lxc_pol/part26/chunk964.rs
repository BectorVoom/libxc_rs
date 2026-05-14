//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 964/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk964<F: Float>(t1248: F, t3979: F, t7744: F, t4054: F, t7938: F, t1237: F, t13669: F, t7931: F, t6043: F, t6051: F, t4037: F, t13632: F, t6059: F, t13672: F, t20295: F, t20440: F, t20504: F, t20510: F) -> (F, F, F, F, F, F, F, F) {
    let t26198 = t1248 * t3979 * t7744;
    let t26203 = t4054 * t7938;
    let t26204 = t26203 * t1237;
    let t26206 = t13669 * t7931;
    let t26207 = t26206 * t1237;
    let t26209 = t6043 * t6051;
    let t26211 = t4037 * t7938;
    let t26212 = t26211 * t1237;
    let t26214 = t13632 * t7931;
    let t26215 = t26214 * t1237;
    let t26217 = t6059 * t6051;
    let t26219 = -t20504 + 0.73028148148148148147e-1 * t20440 - t20510 + 0.13287407407407407407e0 * t20295 + 0.15358125e0 * t26204 + 0.142419375e1 * t26207 - 0.1898925e1 * t26209 - 0.9494625e0 * t26212 - 0.76790625e-1 * t26215 + 0.3071625e0 * t26217 - t13672;
    (t26198, t26204, t26207, t26209, t26212, t26215, t26217, t26219)
}
