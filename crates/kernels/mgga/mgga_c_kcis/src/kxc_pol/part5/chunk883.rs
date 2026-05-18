//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 883/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk883<F: Float>(t1533: F, t7335: F, t4261: F, t6917: F, t4260: F, t143: F, t7028: F, t4219: F, t4220: F, t6281: F, t1517: F, t1650: F, t5987: F) -> (F, F, F, F, F, F) {
    let t7336 = t1533 * t7335;
    let t7338 = t4261 * t6917;
    let t7339 = t4260 * t7338;
    let t7341 = t7028 * t143;
    let t7361 = t4219 * t4220 * t6281;
    let t7365 = t1517 * t5987 * t1650;
    (t7336, t7338, t7339, t7341, t7361, t7365)
}
