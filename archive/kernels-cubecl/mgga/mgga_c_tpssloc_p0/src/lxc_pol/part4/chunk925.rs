//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 925/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk925<F: Float>(t4044: F, t626: F, t4068: F, t2341: F, t92: F, t100: F, t2349: F, t4098: F, t751: F, t172: F, t4095: F, t763: F) -> (F, F, F, F, F, F) {
    let t12750 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t626 * t4044;
    let t12752 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t626 * t4068;
    let t12774 = t92 * t2341;
    let t12795 = t100 * t2349;
    let t12850 = F::cast_from(2.0_f64) * t4098 * t751;
    let t12858 = t4095 * t172;
    let t12860 = F::cast_from(0.11696447245269292414e1_f64) * t12858 * t763;
    (t12750, t12752, t12774, t12795, t12850, t12860)
}
