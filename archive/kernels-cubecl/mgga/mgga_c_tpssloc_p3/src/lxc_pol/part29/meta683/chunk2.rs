//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2316/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2316<F: Float>(t2132: F, t24746: F, t95413: F, t3545: F, t8020: F, t1202: F, t27603: F, t24736: F, t4993: F, t15486: F, t7345: F, t27599: F, t3572: F) -> (F, F, F, F, F, F) {
    let t95446 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t95413 * t24746;
    let t95450 = t8020 * t3545;
    let t95452 = t1202 * t27603;
    let t95456 = t24736 * t4993 / F::cast_from(1728.0_f64);
    let t95459 = t7345 * t15486 / F::cast_from(1728.0_f64);
    let t95463 = t27599 * t3572 / F::cast_from(216.0_f64);
    (t95446, t95450, t95452, t95456, t95459, t95463)
}
