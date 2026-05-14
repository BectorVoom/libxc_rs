//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 517/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk517<F: Float>(t1489: F, t5632: F, t1468: F, t1464: F, t1307: F, t2046: F) -> (F, F, F, F) {
    let t5633 = t5632 * t1489;
    let t5634 = t1468 * t5633;
    let t5635 = t1464 * t5634;
    let t5637 = t2046 * t1307;
    (t5633, t5634, t5635, t5637)
}
