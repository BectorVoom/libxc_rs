//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 561/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk561<F: Float>(t975: F, t978: F, t1014: F, t1088: F, t239: F, t740: F, t313: F, t1031: F, t331: F, t1027: F, t1046: F, t1071: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t3049 = t975 * t978;
    let t3052 = t1014 * t1088;
    let t3054 = t740 * t239;
    let t3056 = F::new(0.46853067927761790996e-2) * t3054 * t313;
    let t3057 = t331 * t1031;
    let t3059 = t1027 * t1046;
    let t3061 = t89 * t1071;
    (t3049, t3052, t3054, t3056, t3057, t3059, t3061)
}
