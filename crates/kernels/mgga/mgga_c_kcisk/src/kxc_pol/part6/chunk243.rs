//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 243/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk243<F: Float>(t1161: F, t303: F, t311: F, t313: F, t436: F, t398: F, t79: F) -> (F, F, F, F, F) {
    let t1178 = 0.29896666666666666667e0 * t1161;
    let t1180 = f64::sqrt(t303);
    let t1184 = t311 * t436 * t313;
    let t1185 = 0.82156666666666666667e-1 * t1184;
    let t1186 = t79 * t398;
    (t1178, t1180, t1184, t1185, t1186)
}
