//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1345/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1345<F: Float>(t32033: F, t6710: F, t6711: F, t6470: F, t9286: F, t32081: F, t544: F, t2365: F, t31752: F, t4391: F, t549: F, t7025: F, t7906: F) -> (F, F, F, F) {
    let t35034 = F::new(0.87421871174939309262e2) * t6710 * t6711 * t32033;
    let t35036 = t9286 * t6470;
    let t35037 = t544 * t32081 * t35036;
    let t35038 = F::new(0.10427226235956374445e0) * t35037;
    let t35040 = t4391 * t2365 * t31752;
    let t35041 = F::new(0.17875244975925213335e0) * t35040;
    let t35043 = t7025 * t549 * t7906;
    (t35034, t35038, t35041, t35043)
}
