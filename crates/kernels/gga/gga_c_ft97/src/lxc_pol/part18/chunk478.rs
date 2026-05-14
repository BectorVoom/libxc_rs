//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 478/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk478<F: Float>(t108: F, t5618: F, t28: F, t1308: F, t497: F, t1314: F, t1882: F, t103: F, t1307: F) -> (F, F, F, F, F, F) {
    let t5619 = t5618 * t108;
    let t5620 = t28 * t5619;
    let t5623 = t1308 * t497;
    let t5624 = t28 * t5623;
    let t5629 = t1882 * t1314 / 9.0;
    let t5630 = t103 * t1307;
    (t5619, t5620, t5623, t5624, t5629, t5630)
}
