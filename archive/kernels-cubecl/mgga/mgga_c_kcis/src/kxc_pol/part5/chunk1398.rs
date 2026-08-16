//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1398/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1398<F: Float>(t18079: F, t21110: F, t1610: F, t6944: F, t4440: F, t21073: F, t6159: F, t21078: F, t6151: F, t531: F, t7429: F, t833: F) -> (F, F, F, F, F) {
    let t23073 = t18079 * t21110;
    let t23076 = t6944 * t1610;
    let t23077 = t4440 * t23076;
    let t23080 = t6159 * t21073;
    let t23083 = t6151 * t21078;
    let t23086 = t7429 * t531;
    let t23087 = t23086 * t833;
    (t23073, t23077, t23080, t23083, t23087)
}
