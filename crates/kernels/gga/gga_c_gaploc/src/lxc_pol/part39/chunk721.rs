//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 721/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk721<F: Float>(t1564: F, t40: F, t6509: F, t9439: F, t9448: F, t585: F, t9419: F, t129: F, t15481: F) -> (F, F, F, F, F, F) {
    let t20550 = t40 * t1564;
    let t20551 = t20550 * t6509;
    let t20556 = t9439 * t6509;
    let t20561 = t9448 * t6509;
    let t20669 = t585 * t9419;
    let t20671 = t15481 * t129;
    (t20550, t20551, t20556, t20561, t20669, t20671)
}
