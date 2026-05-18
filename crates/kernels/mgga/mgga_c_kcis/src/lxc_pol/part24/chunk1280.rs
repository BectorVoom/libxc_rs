//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1280/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1280<F: Float>(t3330: F, t5189: F, t8081: F, t26868: F, t6735: F, t28071: F, t5036: F, t14668: F, t28002: F, t29081: F, t3325: F, t1008: F, t1704: F) -> (F, F, F, F, F, F) {
    let t100945 = F::new(4.0) * t3330 * t8081 * t5189;
    let t100950 = t26868 * t6735;
    let t100952 = F::new(2.0) * t5036 * t28071;
    let t100954 = F::new(4.0) * t14668 * t28002;
    let t100957 = t3325 * t29081;
    let t100970 = t1704 * t1008;
    (t100945, t100950, t100952, t100954, t100957, t100970)
}
