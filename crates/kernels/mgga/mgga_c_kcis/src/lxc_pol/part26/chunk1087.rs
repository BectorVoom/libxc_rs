//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1087/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1087<F: Float>(t28322: F, t446: F, t1299: F, t2132: F, t2233: F, t27364: F, t8164: F, t1394: F, t167: F, t4163: F, t7923: F, t5780: F) -> (F, F, F, F, F, F, F, F) {
    let t28323 = t446 * t28322;
    let t28325 = t1299 * t2132;
    let t28326 = t2233 * t28325;
    let t28328 = t27364 * t8164;
    let t28329 = t1394 * t28328;
    let t28331 = t4163 * t167;
    let t28332 = t7923 * t28331;
    let t28333 = t5780 * t28332;
    (t28323, t28325, t28326, t28328, t28329, t28331, t28332, t28333)
}
