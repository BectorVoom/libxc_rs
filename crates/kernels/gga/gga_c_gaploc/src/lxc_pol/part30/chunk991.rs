//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 991/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk991<F: Float>(t20671: F, t22634: F, t28069: F, t2012: F, t9804: F, t9807: F, t21446: F, t5641: F, t883: F, t9805: F, t1986: F, t9787: F, t1991: F, t9797: F, t21783: F, t3308: F, t6021: F) -> (F, F, F, F, F, F, F, F) {
    let t28072 = 0.85206502119823888169e0 * t28069 * t20671 * t22634;
    let t28073 = t2012 * t9804;
    let t28075 = 0.23005755572352449806e1 * t28073 * t9807;
    let t28079 = 0.23005755572352449806e1 * t9805 * t5641 * t883 * t21446;
    let t28080 = t1986 * t9787;
    let t28081 = 0.1022478025437886658e1 * t28080;
    let t28084 = t1991 * t9797;
    let t28085 = 0.2044956050875773316e1 * t28084;
    let t28089 = 0.11502877786176224903e1 * t9805 * t5641 * t883 * t21783;
    let t28099 = t6021 * t3308;
    (t28072, t28073, t28075, t28079, t28081, t28085, t28089, t28099)
}
