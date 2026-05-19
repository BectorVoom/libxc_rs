//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1156/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1156<F: Float>(t2472: F, t3802: F, t7604: F, t845: F, t2367: F, t7300: F, t999: F, t2441: F, t7344: F, t2435: F, t2436: F, t7266: F) -> (F, F, F, F) {
    let t24037 = F::cast_from(0.69263023597503453196e2_f64) * t845 * t2472 * t7604 * t3802;
    let t24041 = t999 * t2367 * t7300;
    let t24044 = F::cast_from(0.4155781415850207192e3_f64) * t2441 * t7344;
    let t24046 = t2435 * t2436 * t7266;
    (t24037, t24041, t24044, t24046)
}
