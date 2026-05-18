//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1089/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1089<F: Float>(t1986: F, t9787: F, t1991: F, t9797: F, t21783: F, t5641: F, t883: F, t9805: F, t3308: F, t6021: F, t165: F, t5397: F, t935: F) -> (F, F, F, F, F) {
    let t28080 = t1986 * t9787;
    let t28081 = F::new(0.1022478025437886658e1) * t28080;
    let t28084 = t1991 * t9797;
    let t28085 = F::new(0.2044956050875773316e1) * t28084;
    let t28089 = F::new(0.11502877786176224903e1) * t9805 * t5641 * t883 * t21783;
    let t28099 = t6021 * t3308;
    let t28126 = t165 * t935 * t5397;
    (t28081, t28085, t28089, t28099, t28126)
}
