//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 640/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk640<F: Float>(t15879: F, t409: F, t64: F, t11232: F, t11233: F, t15689: F, t15797: F, t15802: F, t15806: F, t15811: F, t15812: F, t15819: F, t15822: F, t15825: F, t15829: F, t1624: F, t372: F, t6426: F, t7845: F, t7877: F, t7985: F, t7989: F) -> (F,) {
    let t15881 = t64 * t409 * t15879;
    let t15882 = 0.13519760450715832853e-3 * t15797 * t7985 - 0.67598802253579164263e-4 * t15797 * t7989 + 0.13784064983740990796e-3 * t7845 * t15802 + 0.46509801892875584e-1 * t7877 * t6426 * t15806 - 0.46509801892875584e-1 * t15811 * t6426 * t15812 - 0.46509801892875584e-2 * t11232 * t11233 * t15689 - 0.23254900946437792e-2 * t1624 * t15819 - 0.279058811357253504e-2 * t372 * t15822 + 0.46509801892875584e-2 * t372 * t15825 + 0.23254900946437792e-1 * t1624 * t15829 - t15881;
    (t15882,)
}
