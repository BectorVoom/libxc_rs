//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 556/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk556<F: Float>(t1195: F, t1812: F, t1187: F, t3438: F, t4823: F, t3437: F, t143: F, t4768: F, t365: F, t932: F, t346: F, t1110: F, t1115: F, t1143: F, t1757: F, t1761: F, t1780: F, t3381: F, t348: F, t4602: F, t4607: F, t4626: F, t4638: F, t4643: F, t4671: F) -> (F, F, F, F, F, F, F, F) {
    let t5096 = t1195 * t1812;
    let t5097 = t1187 * t5096;
    let t5099 = t3438 * t4823;
    let t5100 = t3437 * t5099;
    let t5102 = t4768 * t143;
    let t5111 = t365 * t932;
    let t5122 = t365 * t346;
    let t5127 = F::cast_from(0.619125e-2_f64) * t5102 * t348 + F::cast_from(0.9286875e-2_f64) * t1780 * t1110 - F::cast_from(0.619125e-2_f64) * t1780 * t1115 + F::cast_from(0.9286875e-2_f64) * t1143 * t1757 + F::cast_from(0.46434375e-2_f64) * t5111 * t4602 - F::cast_from(0.9286875e-2_f64) * t3381 * t4607 + F::cast_from(0.9286875e-2_f64) * t365 * t4626 - F::cast_from(0.619125e-2_f64) * t1143 * t1761 - F::cast_from(0.9286875e-2_f64) * t3381 * t4638 + F::cast_from(0.123825e-1_f64) * t5122 * t4643 - F::cast_from(0.619125e-2_f64) * t365 * t4671;
    (t5096, t5097, t5099, t5100, t5102, t5111, t5122, t5127)
}
