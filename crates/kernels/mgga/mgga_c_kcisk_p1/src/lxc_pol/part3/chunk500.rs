//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 500/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk500<F: Float>(t3532: F, t403: F, t3278: F, t3952: F, t1318: F, t402: F, t398: F, t1322: F, t1310: F, t1293: F, t1308: F) -> (F, F, F, F, F, F, F, F) {
    let t3953 = t403 * t3532;
    let t3954 = t3953 * t3278;
    let t3955 = t3952 * t3954;
    let t3959 = F::new(1.0) / t1318 / t402;
    let t3960 = t398 * t3959;
    let t3961 = t1322 * t1322;
    let t3962 = t3960 * t3961;
    let t3963 = t1310 * t3962;
    let t3966 = t1293 * t1308;
    (t3953, t3954, t3955, t3959, t3961, t3962, t3963, t3966)
}
