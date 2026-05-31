//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 418/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk418<F: Float>(t206: F, t2726: F, t2728: F, t20: F, t2394: F, t62: F, t212: F, t879: F, t882: F, t209: F, t2718: F) -> (F, F, F, F, F, F) {
    let t210 = F::cast_from(0.0_f64) < t206;
    let t2729 = t2726 * t2728;
    let t2733 = t62 * t2394 * t20;
    let t2739 = F::cast_from(1.0_f64) / t879 / t212;
    let t2740 = t882 * t882;
    let t2742 = t209 * t2739 * t2740;
    let t2746 = piecewise3::<F>(t210, t2718, -t2718);
    (t2729, t2733, t2739, t2740, t2742, t2746)
}
