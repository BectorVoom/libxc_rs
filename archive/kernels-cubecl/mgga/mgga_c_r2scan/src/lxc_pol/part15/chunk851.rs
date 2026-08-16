//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 851/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk851<F: Float>(t1831: F, t963: F, t2747: F, t750: F, t1842: F, t1814: F, t5249: F, t897: F, t5252: F, t5298: F, t5302: F, t5303: F, t5307: F, t5321: F, t5323: F, t5327: F) -> F {
    let t7685 = t963 * t1831;
    let t7688 = F::cast_from(0.34631718211362927518e2_f64) * t2747 * t750;
    let t7689 = t963 * t1842;
    let t7691 = t963 * t1814;
    let t7693 = t5249 * t897;
    let t7694 = t7693 * t5252;
    let t7696 = t5298 + t5302 + F::cast_from(0.34631718211362927518e2_f64) * t5303 + t5307 + t5321 + F::cast_from(0.2701041328e0_f64) * t5323 + F::cast_from(0.2701041328e0_f64) * t5327 - F::cast_from(0.11696447245269292414e1_f64) * t7685 + t7688 + F::cast_from(0.34631718211362927518e2_f64) * t7689 + F::cast_from(0.17315859105681463759e2_f64) * t7691 - F::cast_from(0.4051561992e0_f64) * t7694;
    t7696
}
