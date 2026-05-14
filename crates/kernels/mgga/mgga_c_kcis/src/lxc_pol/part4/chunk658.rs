//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 658/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk658<F: Float>(t1396: F, t3954: F, t1468: F, t1464: F, t1362: F, t506: F, t486: F) -> (F, F, F, F, F) {
    let t3955 = t1396 * t3954;
    let t3956 = t1468 * t3955;
    let t3957 = t1464 * t3956;
    let t3960 = 1.0 / t1362 / t506;
    let t3961 = t486 * t3960;
    (t3955, t3956, t3957, t3960, t3961)
}
