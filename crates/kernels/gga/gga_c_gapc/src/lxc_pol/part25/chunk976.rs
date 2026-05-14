//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 976/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk976<F: Float>(t1030: F, t26312: F, t20487: F, t3141: F, t3131: F, t3137: F, t15341: F, t8676: F, t128: F, t1453: F, t134: F, t681: F, t5216: F, t1673: F, t9255: F, t13483: F) -> (F, F, F, F, F, F, F, F) {
    let t26561 = t1030 * t26312;
    let t26578 = t3141 * t20487;
    let t26597 = t3131 * t3137;
    let t26609 = t8676 * t15341;
    let t26662 = t128 * t1453;
    let t26697 = t681 * t134;
    let t26698 = t26697 * t5216;
    let t26759 = t1673 * t9255;
    let t26778 = t8676 * t13483;
    (t26561, t26578, t26597, t26609, t26662, t26698, t26759, t26778)
}
