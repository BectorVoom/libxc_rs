//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 887/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk887<F: Float>(t1036: F, t1089: F, t175: F, t839: F, t864: F, t1103: F, t3244: F, t1005: F, t3493: F, t3292: F, t3101: F, t360: F, t368: F, t384: F, t398: F) -> (F, F, F, F, F) {
    let t13133 = t1036 * t1089 * t175 * t864 * t839;
    let t13135 = t3244 * t1103;
    let t13137 = t1005 * t3493;
    let t13146 = t1005 * t3292;
    let t13156 = t384 * t398 * t368 * t3101 * t360;
    (t13133, t13135, t13137, t13146, t13156)
}
