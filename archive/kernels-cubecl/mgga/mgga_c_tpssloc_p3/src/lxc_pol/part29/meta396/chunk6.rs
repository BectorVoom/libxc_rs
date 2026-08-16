//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1628/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1628<F: Float>(t11593: F, t4904: F, t11570: F, t3961: F, t11569: F, t1174: F, t15332: F, t15335: F, t15341: F, t15360: F, t15364: F, t15366: F, t15374: F, t15376: F, t3447: F, t3452: F, t3472: F, t3478: F, t4889: F) -> F {
    let t15379 = t11593 * t4904;
    let t15382 = t11570 * t3961;
    let t15383 = t11569 * t15382;
    let t15386 = -F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t15332 - F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t15335 + t15341 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t15360 + F::cast_from(0.18518518518518518518e-3_f64) * t15364 + F::cast_from(0.14814814814814814815e-2_f64) * t15366 + F::cast_from(0.22222222222222222222e-2_f64) * t4889 * t3472 + F::cast_from(0.22222222222222222222e-2_f64) * t4889 * t3478 - t15374 - F::cast_from(0.14814814814814814815e-2_f64) * t15376 * t3452 + F::cast_from(0.27777777777777777777e-3_f64) * t3447 * t15379 - F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t15383;
    t15386
}
