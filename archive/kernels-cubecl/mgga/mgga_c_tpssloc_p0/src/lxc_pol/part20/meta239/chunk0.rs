//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1340/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1340<F: Float>(t831: F, t9674: F, t2639: F, t2681: F, t184: F, t2250: F, t607: F, t4194: F, t116: F, t126: F, t136: F, t16: F) -> (F, F, F, F, F, F) {
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9681 = t184 * t2250;
    let t9682 = t9681 * t607;
    let t9684 = F::cast_from(36.0_f64) * t4194 * t9682;
    let t9688 = F::cast_from(1.0_f64) / t126 / t136 * t116 / F::cast_from(4.0_f64);
    let t9689 = t9688 * t16;
    (t9675, t9679, t9682, t9684, t9688, t9689)
}
