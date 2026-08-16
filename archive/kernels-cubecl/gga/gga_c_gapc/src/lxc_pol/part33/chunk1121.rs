//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1121/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1121<F: Float>(t11804: F, t11814: F, t19210: F, t15699: F, t7502: F, t9895: F, t15680: F, t26597: F, t7259: F, t11986: F, t3367: F, t6182: F) -> (F, F, F, F) {
    let t33914 = t11814 * t11804 * t19210;
    let t33917 = t9895 * t7502 * t15699;
    let t33920 = t7259 * t26597 * t15680;
    let t33923 = t11986 * t3367 * t6182;
    (t33914, t33917, t33920, t33923)
}
