//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1056/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1056<F: Float>(t11258: F, t3946: F, t514: F, t1005: F, t13736: F, t3639: F, t4885: F, t11273: F, t8451: F, t25526: F, t3643: F, t3646: F, t11270: F, t25530: F, t11262: F, t8562: F) -> (F, F, F, F, F, F) {
    let t35400 = t514 * t3946 * t11258;
    let t35404 = t1005 * t13736 * t3639 * t4885;
    let t35406 = t8451 * t11273;
    let t35409 = t3643 * t25526 * t3646;
    let t35412 = t11270 * t25530 * t11273;
    let t35415 = t8562 * t11262;
    (t35400, t35404, t35406, t35409, t35412, t35415)
}
