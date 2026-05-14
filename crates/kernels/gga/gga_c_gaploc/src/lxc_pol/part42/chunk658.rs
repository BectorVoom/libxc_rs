//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 658/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk658<F: Float>(t1564: F, t165: F, t10524: F, t1415: F, t1433: F, t9271: F, t40: F, t6509: F, t9439: F, t9448: F, t585: F, t9419: F, t129: F, t15481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20368 = t165 * t1564;
    let t20471 = t1415 * t10524;
    let t20535 = t1433 * t9271;
    let t20550 = t40 * t1564;
    let t20551 = t20550 * t6509;
    let t20556 = t9439 * t6509;
    let t20561 = t9448 * t6509;
    let t20669 = t585 * t9419;
    let t20671 = t15481 * t129;
    (t20368, t20471, t20535, t20550, t20551, t20556, t20561, t20669, t20671)
}
