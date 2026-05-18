//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1159/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1159<F: Float>(t28123: F, t4547: F, t5302: F, t1262: F, t1856: F, t26996: F, t5329: F) -> (F, F, F, F) {
    let t28124 = t28123 * t4547;
    let t28125 = t5302 * t28124;
    let t28130 = t1856 * t1262;
    let t28131 = t26996 * t28130;
    let t28132 = t5329 * t28131;
    (t28124, t28125, t28131, t28132)
}
