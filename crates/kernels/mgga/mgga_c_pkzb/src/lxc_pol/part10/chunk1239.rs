//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1239/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1239<F: Float>(t1719: F, t3441: F, t164: F, t20010: F, t8955: F, t5257: F, t8901: F, t6892: F, t8950: F, t1753: F, t8948: F, t3410: F, t1731: F, t1774: F, t1034: F, t616: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24210 = t3441 * t1719;
    let t24211 = t24210 * t164;
    let t24215 = t20010 * t8955;
    let t24217 = t5257 * t8901;
    let t24219 = t6892 * t8950;
    let t24221 = t8948 * t1753;
    let t24226 = t3410 * t1753 * t164;
    let t24235 = t1731 * t1774;
    let t24237 = t616 * t1034;
    (t24210, t24211, t24215, t24217, t24219, t24221, t24226, t24235, t24237)
}
