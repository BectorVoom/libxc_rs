//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1151/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1151<F: Float>(t23041: F, t831: F, t2686: F, t6614: F, t2627: F, t59: F, t240: F, t812: F, t2635: F, t2681: F, t2617: F, t6613: F) -> (F, F, F, F, F, F, F, F) {
    let t23042 = t23041 * t831;
    let t23043 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t23042;
    let t23044 = t6614 * t2686;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23049 = t23048 * t2635;
    let t23051 = t6614 * t2681;
    let t23053 = t2617 * t6613;
    (t23043, t23044, t23046, t23047, t23048, t23049, t23051, t23053)
}
