//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2450/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2450<F: Float>(t1340: F, t40182: F, t39821: F, t40196: F, t40192: F, t4038: F, t9419: F, t40113: F, t40169: F, t3863: F, t4029: F, t40135: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47084 = F::cast_from(0.5848223622634646207e0_f64) * t1340 * t40182;
    let t47086 = F::cast_from(0.61524113149298439947e4_f64) * t1340 * t39821;
    let t47088 = F::cast_from(0.35089341735807877242e1_f64) * t1340 * t40196;
    let t47092 = F::cast_from(0.14035736694323150897e2_f64) * t1340 * t40192;
    let t47093 = t4038 * t9419;
    let t47096 = F::cast_from(0.51947577317044391277e2_f64) * t1340 * t40113;
    let t47098 = F::cast_from(0.91082604192152556044e5_f64) * t1340 * t40169;
    let t47101 = t3863 * t4029;
    let t47109 = F::cast_from(0.6233709278045326953e3_f64) * t1340 * t40135;
    (t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47101, t47109)
}
