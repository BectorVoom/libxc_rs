//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1274/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1274<F: Float>(t4389: F, t5859: F, t1181: F, t12991: F, t4347: F, t530: F, t4396: F, t6332: F, t12930: F, t1761: F, t3409: F, t5807: F) -> (F, F, F, F, F) {
    let t23568 = t4389 * t5859;
    let t23572 = t12991 * t1181 * t530 * t4347;
    let t23574 = t4396 * t6332;
    let t23584 = t12930 * t1761;
    let t23586 = t3409 * t5807;
    (t23568, t23572, t23574, t23584, t23586)
}
