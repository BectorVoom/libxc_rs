//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1054/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1054<F: Float>(t27514: F, t7949: F, t1528: F, t491: F, t7953: F, t4254: F) -> (F, F, F, F) {
    let t27515 = t27514 * t7949;
    let t27517 = t1528 * t491;
    let t27518 = t27517 * t7953;
    let t27520 = t4254 * t491;
    (t27515, t27517, t27518, t27520)
}
