//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 635/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk635<F: Float>(t303: F, t7108: F, t518: F, t6281: F, t1961: F, t5792: F, t6284: F) -> (F, F, F, F, F) {
    let t7109 = t303 * t7108;
    let t7113 = t518 * t6281;
    let t7116 = t5792 * t1961;
    let t7119 = t518 * t6284;
    let t7122 = t1961 * t1961;
    (t7109, t7113, t7116, t7119, t7122)
}
