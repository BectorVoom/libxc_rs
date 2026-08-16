//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 723/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk723<F: Float>(t8545: F, t996: F, t2912: F, t4538: F, t2929: F, t1599: F, t2932: F, t2958: F, t1577: F, t8399: F, t2936: F, t2937: F, t4644: F) -> (F, F, F, F, F) {
    let t8546 = t996 * t8545;
    let t8547 = t8546 * t2912;
    let t8549 = t996 * t4538;
    let t8550 = t8549 * t2929;
    let t8552 = t2932 * t1599;
    let t8553 = t8552 * t2958;
    let t8556 = t8399 * t1577;
    let t8557 = t2936 * t8556;
    let t8559 = t2937 * t4644;
    (t8547, t8550, t8553, t8557, t8559)
}
