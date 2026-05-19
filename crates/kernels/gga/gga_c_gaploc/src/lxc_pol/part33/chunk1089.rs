//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1089/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1089<F: Float>(t28073: F, t9807: F, t21446: F, t5641: F, t883: F, t9805: F, t1986: F, t9787: F, t1991: F, t9797: F, t21783: F, t3308: F, t6021: F) -> (F, F, F, F, F, F) {
    let t28075 = F::cast_from(0.23005755572352449806e1_f64) * t28073 * t9807;
    let t28079 = F::cast_from(0.23005755572352449806e1_f64) * t9805 * t5641 * t883 * t21446;
    let t28080 = t1986 * t9787;
    let t28084 = t1991 * t9797;
    let t28089 = F::cast_from(0.11502877786176224903e1_f64) * t9805 * t5641 * t883 * t21783;
    let t28099 = t6021 * t3308;
    (t28075, t28079, t28080, t28084, t28089, t28099)
}
