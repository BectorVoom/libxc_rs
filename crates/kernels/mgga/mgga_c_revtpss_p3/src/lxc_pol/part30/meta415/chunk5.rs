//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1563/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1563<F: Float>(t11528: F, t4595: F, t11294: F, t4636: F, t4632: F, t934: F, t2874: F, t1610: F, t2918: F, t2875: F, t4635: F, t11299: F) -> (F, F, F, F, F) {
    let t15377 = F::cast_from(4.0_f64) * t11528 * t4595;
    let t15379 = F::cast_from(0.32163958997385070134e2_f64) * t11294 * t4636;
    let t15380 = t4632 * t934;
    let t15382 = F::cast_from(4.0_f64) * t2874 * t15380;
    let t15383 = t1610 * t2918;
    let t15385 = F::cast_from(2.0_f64) * t2874 * t15383;
    let t15386 = t4635 * t2875;
    let t15388 = F::cast_from(0.96491876992155210402e2_f64) * t11299 * t15386;
    (t15377, t15379, t15382, t15385, t15388)
}
