//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 308/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk308<F: Float>(t1440: F, t470: F, t487: F, t1487: F, t1299: F, t41: F) -> (F, F, F, F) {
    let t1488 = t470 * t1440;
    let t1489 = t487 * t1488;
    let t1490 = t1487 * t1489;
    let t1492 = t1299 * t41;
    (t1488, t1489, t1490, t1492)
}
