//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 996/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk996<F: Float>(t1446: F, t3237: F, t4724: F, t997: F, t5113: F, t3670: F, t1032: F, t4503: F, t4625: F, t1181: F, t16507: F, t3361: F, t4267: F) -> (F, F, F, F, F, F, F) {
    let t16637 = t3237 * t1446;
    let t16639 = t997 * t4724;
    let t16641 = t997 * t5113;
    let t16644 = t3670 * t1446;
    let t16646 = t1032 * t4503;
    let t16648 = t1032 * t4625;
    let t16663 = t3361 * t1181 * t4267 * t16507;
    (t16637, t16639, t16641, t16644, t16646, t16648, t16663)
}
