//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1030/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1030<F: Float>(t5274: F, t5278: F, t5282: F, t5288: F, t5295: F, t5298: F, t5302: F, t5307: F, t5321: F, t7685: F, t7689: F, t7691: F, t7694: F, t7699: F, t7701: F, t7703: F, t8909: F, t8917: F) -> (F,) {
    let t10217 = t5274 - t5278 + t5282 - t5288 - t5295 + t5298 + t5302 + t5307 + t5321 - 0.35089341735807877242e1 * t7685 + 0.10389515463408878255e3 * t7689 + 0.51947577317044391277e2 * t7691 - 0.12154685976e1 * t7694 + 0.4051561992e0 * t7699 - 0.16265371950452609763e-1 * t8909 - 36.0 * t7701 - 96.0 * t7703 + 0.254044196e-2 * t8917;
    (t10217,)
}
