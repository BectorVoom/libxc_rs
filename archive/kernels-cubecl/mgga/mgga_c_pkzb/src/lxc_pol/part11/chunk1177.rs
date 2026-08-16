//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1177/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1177<F: Float>(t24527: F, t10731: F, t639: F, t16554: F, t16571: F, t16582: F, t24534: F, t24536: F, t24539: F, t24542: F, t16584: F, t24600: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28942 = F::cast_from(12.0_f64) * t24527;
    let t28943 = t10731 * t639;
    let t28950 = F::cast_from(0.5848223622634646207e0_f64) * t16554;
    let t28951 = F::cast_from(120.0_f64) * t16571;
    let t28952 = F::cast_from(0.48159733137676571078e0_f64) * t16582;
    let t28954 = F::cast_from(60.0_f64) * t24534;
    let t28955 = F::cast_from(0.51947577317044391276e2_f64) * t24536;
    let t28956 = F::cast_from(0.17544670867903938621e1_f64) * t24539;
    let t28957 = F::cast_from(0.17544670867903938621e1_f64) * t24542;
    let t28958 = F::cast_from(12.0_f64) * t16584;
    let t28959 = F::cast_from(3.0_f64) * t24600;
    (t28942, t28943, t28950, t28951, t28952, t28954, t28955, t28956, t28957, t28958, t28959)
}
