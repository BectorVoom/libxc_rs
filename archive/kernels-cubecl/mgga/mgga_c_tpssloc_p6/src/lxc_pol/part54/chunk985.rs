//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 985/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk985<F: Float>(t10109: F, t1911: F, t4272: F, t25036: F, t25042: F, t25047: F, t25049: F, t25051: F, t25056: F, t25061: F, t25161: F, t25168: F, t259: F, t2597: F, t4147: F, t4301: F, t6627: F, t6632: F, t6663: F, t7538: F) -> (F, F) {
    let t25169 = t10109 * t1911;
    let t25170 = t25169 * t4272;
    let t25173 = -F::cast_from(0.41123351671205660912e-2_f64) * t25036 + F::cast_from(0.49348022005446793095e-1_f64) * t25042 + F::cast_from(0.16449340668482264365e-1_f64) * t25047 - F::cast_from(0.19190897446562641759e-1_f64) * t25049 + t25051 * t259 + F::cast_from(0.16449340668482264365e-1_f64) * t25056 + F::cast_from(0.82246703342411321825e-2_f64) * t25061 + t25161 * t259 - t2597 * t7538 - t6627 * t4301 + F::cast_from(2.0_f64) * t4147 * t6632 - t4147 * t6663 - F::cast_from(6.0_f64) * t25168 * t25170;
    (t25170, t25173)
}
