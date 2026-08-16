//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1191/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1191<F: Float>(t15699: F, t7502: F, t9895: F, t15680: F, t26597: F, t7259: F, t11986: F, t3367: F, t6182: F, t2268: F, t3438: F, t3439: F) -> (F, F, F, F) {
    let t33917 = t9895 * t7502 * t15699;
    let t33920 = t7259 * t26597 * t15680;
    let t33923 = t11986 * t3367 * t6182;
    let t33928 = t3438 * t2268 * t3439;
    (t33917, t33920, t33923, t33928)
}
