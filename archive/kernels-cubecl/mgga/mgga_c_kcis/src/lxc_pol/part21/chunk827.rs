//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 827/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk827<F: Float>(t1018: F, t127: F, t368: F, t245: F, t313: F, t330: F, t1098: F, t3305: F, t1111: F, t3251: F, t1116: F, t3300: F) -> (F, F, F, F, F, F, F) {
    let t10414 = t127 * t368 * t1018;
    let t10415 = t245 * t313;
    let t10416 = t10415 * t330;
    let t10422 = t1098 * t3305;
    let t10424 = t3251 * t1111;
    let t10426 = t3251 * t1116;
    let t10428 = t1098 * t3300;
    (t10414, t10415, t10416, t10422, t10424, t10426, t10428)
}
