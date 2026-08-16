//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 856/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk856<F: Float>(t3515: F, t655: F, t218: F, t219: F, t208: F, t9161: F, t5558: F, t5560: F, t7332: F, t7465: F, t7466: F, t9178: F, t9180: F, t9185: F, t9189: F, t9192: F) -> (F, F, F, F, F) {
    let t9194 = t655 * t3515;
    let t9196 = t218 * t219 * t9194;
    let t9198 = t208 * t9161;
    let t9200 = t218 * t219 * t9198;
    let t9202 = F::cast_from(0.82524375e-1_f64) * t9178 + F::cast_from(0.16504875e0_f64) * t9180 - t5558 + F::cast_from(0.27595e0_f64) * t5560 + F::cast_from(0.5519e0_f64) * t7332 - t7465 - t7466 - F::cast_from(0.16557e0_f64) * t9185 + F::cast_from(0.49671e0_f64) * t9189 - F::cast_from(0.16557e0_f64) * t9192 + F::cast_from(0.248355e0_f64) * t9196 + F::cast_from(0.248355e0_f64) * t9200;
    (t9194, t9196, t9198, t9200, t9202)
}
