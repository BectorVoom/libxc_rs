//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 287/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk287<F: Float>(t1163: F, t1341: F, t1340: F, t1339: F, t167: F, t300: F) -> (F, F, F, F) {
    let t1342 = t1341 * t1163;
    let t1343 = t1340 * t1342;
    let t1344 = t1339 * t1343;
    let t1346 = t167 * t300;
    (t1342, t1343, t1344, t1346)
}
