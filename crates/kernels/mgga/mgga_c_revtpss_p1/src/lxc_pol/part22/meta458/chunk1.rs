//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2132/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2132<F: Float>(t11456: F, t15350: F, t15373: F, t15377: F, t15379: F, t15382: F, t15385: F, t15388: F, t15392: F, t15395: F, t15399: F, t15400: F, t1634: F, t2982: F, t3015: F, t311: F, t4708: F, t955: F) -> F {
    let t15403 = F::cast_from(0.17315859105681463759e2_f64) * t15350 * t3015 + F::cast_from(0.5848223622634646207e0_f64) * t11456 * t1634 + F::cast_from(0.11696447245269292414e1_f64) * t2982 * t4708 - F::cast_from(0.310907e-1_f64) * t15373 * t311 + t15377 - t15379 + t15382 + t15385 + t15388 - t15392 - t15395 - t15399 + F::cast_from(2.0_f64) * t15400 * t955;
    t15403
}
