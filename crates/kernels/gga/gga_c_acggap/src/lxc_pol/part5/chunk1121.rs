//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1121/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1121<F: Float>(t6021: F, t691: F, t288: F, t5474: F, t75: F, t682: F, t11945: F, t1708: F, t4: F, t657: F, t12157: F, t12665: F) -> (F, F, F, F, F, F, F) {
    let t20042 = t6021 * t691;
    let t20043 = F::cast_from(0.17315859105681463759e2_f64) * t20042;
    let t20045 = t5474 * t75 * t288;
    let t20046 = F::cast_from(0.11696447245269292414e1_f64) * t20045;
    let t20047 = t6021 * t682;
    let t20048 = F::cast_from(0.5848223622634646207e0_f64) * t20047;
    let t20049 = F::cast_from(0.20508037716432813316e4_f64) * t11945;
    let t20051 = t1708 * t4 * t657;
    let t20052 = F::cast_from(0.10843581300301739842e-1_f64) * t20051;
    let t20053 = F::new(24.0) * t12157;
    let t20054 = F::cast_from(0.65061487801810439052e-1_f64) * t12665;
    (t20043, t20046, t20048, t20049, t20052, t20053, t20054)
}
