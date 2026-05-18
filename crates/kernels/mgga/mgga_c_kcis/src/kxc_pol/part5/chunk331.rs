//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 331/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk331<F: Float>(t20: F, t982: F, t414: F, t24: F, t287: F, t209: F, t421: F, t416: F, t415: F, t68: F) -> (F, F, F, F, F, F) {
    let t1241 = t982 * t20;
    let t1242 = t414 * t1241;
    let t1245 = t24 * t287;
    let t1247 = t209 * t1245 * t421;
    let t1249 = t416 * t1247 / F::new(576.0);
    let t1250 = t415 * t68;
    let t1251 = t414 * t1250;
    (t1241, t1242, t1245, t1247, t1249, t1251)
}
