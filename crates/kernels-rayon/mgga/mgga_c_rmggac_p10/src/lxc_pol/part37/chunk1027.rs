//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1027/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1027(t70618: f64, t76550: f64, t14387: f64, t14389: f64, t14393: f64, t14398: f64, t14399: f64, t14400: f64, t15051: f64, t15426: f64, t15427: f64, t15428: f64, t15429: f64, t15430: f64, t15856: f64, t15857: f64, t70657: f64) -> (f64, f64, f64) {
    let t78612 = 0.16263363996404810741e-4_f64 * t70618;
    let t78613 = 0.14967802127329760705e-1_f64 * t76550;
    let t79943 = -t15856 - t15857 + t15426 + t15427 - t15428 - t15429 + t15051 + t14387 - t14389 + t14393 + t14398 - t14399 + t14400 + t70657 - t15430;
    (t78612, t78613, t79943)
}
