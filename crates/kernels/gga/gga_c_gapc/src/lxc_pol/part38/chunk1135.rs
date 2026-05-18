//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1135/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1135<F: Float>(t1084: F, t28517: F, t34077: F, t1044: F, t825: F, t19: F, t311: F, t3752: F, t10293: F, t28192: F, t33399: F, t9894: F) -> (F, F, F, F) {
    let t34079 = t1084 * t34077 * t28517;
    let t34081 = t825 * t1044;
    let t34084 = t311 * t34081 * t19 * t3752;
    let t34088 = t9894 * t33399 * t10293 * t28192;
    (t34079, t34081, t34084, t34088)
}
