//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 686/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk686<F: Float>(t1616: F, t481: F, t2207: F, t785: F, t1610: F, t2201: F, t2202: F, t2208: F, t239: F, t4715: F, t5: F, t1398: F, t753: F) -> (F, F, F, F, F) {
    let t5181 = t1616 * t481;
    let t5183 = t2207 * t785 * t5181;
    let t5186 = t2201 * t1610 * t2202;
    let t5189 = t2207 * t1610 * t2208;
    let t5193 = F::new(140.0) / F::new(27.0) * t5 * t4715 * t239;
    let t5195 = t5 * t1398 * t753;
    (t5183, t5186, t5189, t5193, t5195)
}
