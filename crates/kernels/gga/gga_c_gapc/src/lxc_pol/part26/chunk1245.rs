//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1245/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1245<F: Float>(t11258: F, t3946: F, t514: F, t1005: F, t13736: F, t3639: F, t4885: F, t11273: F, t8451: F, t25526: F, t3643: F, t3646: F) -> (F, F, F, F) {
    let t35400 = t514 * t3946 * t11258;
    let t35404 = t1005 * t13736 * t3639 * t4885;
    let t35406 = t8451 * t11273;
    let t35409 = t3643 * t25526 * t3646;
    (t35400, t35404, t35406, t35409)
}
