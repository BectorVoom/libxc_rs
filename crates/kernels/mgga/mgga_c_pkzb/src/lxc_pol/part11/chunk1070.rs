//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1070/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1070<F: Float>(t24527: F, t10731: F, t639: F, t16554: F, t16571: F, t16582: F, t24534: F, t24536: F, t24539: F, t24542: F, t16584: F, t24600: F, t24604: F, t24606: F, t16593: F, t16595: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28942 = 12.0 * t24527;
    let t28943 = t10731 * t639;
    let t28950 = 0.5848223622634646207e0 * t16554;
    let t28951 = 120.0 * t16571;
    let t28952 = 0.48159733137676571078e0 * t16582;
    let t28954 = 60.0 * t24534;
    let t28955 = 0.51947577317044391276e2 * t24536;
    let t28956 = 0.17544670867903938621e1 * t24539;
    let t28957 = 0.17544670867903938621e1 * t24542;
    let t28958 = 12.0 * t16584;
    let t28959 = 3.0 * t24600;
    let t28960 = 0.32530743900905219526e-1 * t24604;
    let t28961 = 0.35089341735807877242e1 * t24606;
    let t28962 = 0.35089341735807877242e1 * t16593;
    let t28963 = 0.21687162600603479684e-1 * t16595;
    (t28942, t28943, t28950, t28951, t28952, t28954, t28955, t28956, t28957, t28958, t28959, t28960, t28961, t28962, t28963)
}
