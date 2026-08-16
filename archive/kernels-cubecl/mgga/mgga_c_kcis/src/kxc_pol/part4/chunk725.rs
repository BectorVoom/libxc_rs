//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 725/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk725<F: Float>(t4243: F, t552: F, t573: F, t1466: F, t1527: F, t1535: F, t1529: F, t1539: F, t4121: F, t569: F, t4124: F, t556: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4244 = t4243 * t552;
    let t4245 = t4244 * sigma2;
    let t4246 = t4245 * t573;
    let t4248 = t1527 * t1466;
    let t4249 = t4248 * sigma2;
    let t4250 = t4249 * t1535;
    let t4252 = t1529 * t1539;
    let t4254 = t569 * t4121;
    let t4255 = t4254 * sigma2;
    let t4256 = t556 * t4124;
    (t4245, t4246, t4248, t4249, t4250, t4252, t4254, t4255, t4256)
}
