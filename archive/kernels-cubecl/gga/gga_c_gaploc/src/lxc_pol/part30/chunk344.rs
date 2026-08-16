//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 344/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk344<F: Float>(t163: F, t20: F, t1476: F, t14: F, t72: F, t506: F, t397: F, t4: F, t78: F, t3: F, t97: F, t508: F) -> (F, F, F, F, F, F) {
    let t1477 = t20 * t163;
    let t1478 = t1476 * t1477;
    let t1481 = t14 * t72;
    let t1482 = t506 * t1481;
    let t1484 = t4 * t78 * t397;
    let t1487 = t3 * t97;
    let t1488 = t508 * t1487;
    (t1477, t1478, t1482, t1484, t1487, t1488)
}
