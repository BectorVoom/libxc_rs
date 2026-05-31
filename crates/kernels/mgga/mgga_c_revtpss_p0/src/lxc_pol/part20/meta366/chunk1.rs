//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1339/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1339<F: Float>(t10558: F, t177: F, t762: F, t150: F, t190: F, t39854: F, t2491: F, t2495: F, t39871: F, t760: F, t10433: F, t2398: F) -> (F, F, F, F, F) {
    let t40108 = t10558 * t177 * t762;
    let t40109 = F::cast_from(0.23392894490538584828e1_f64) * t40108;
    let t40111 = t150 * t39854 * t190;
    let t40113 = t2491 * t39871 * t2495;
    let t40115 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t40113;
    let t40117 = F::cast_from(16.0_f64) * t2398 * t10433;
    (t40109, t40111, t40113, t40115, t40117)
}
