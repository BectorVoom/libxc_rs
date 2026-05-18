//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1290/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1290<F: Float>(t25526: F, t3643: F, t3646: F, t11270: F, t11273: F, t25530: F, t11262: F, t8562: F, t11235: F, t15355: F, t15358: F, t3650: F) -> (F, F, F, F) {
    let t35409 = t3643 * t25526 * t3646;
    let t35412 = t11270 * t25530 * t11273;
    let t35415 = t8562 * t11262;
    let t35419 = t3650 * t15355 * t11235 * t15358;
    (t35409, t35412, t35415, t35419)
}
