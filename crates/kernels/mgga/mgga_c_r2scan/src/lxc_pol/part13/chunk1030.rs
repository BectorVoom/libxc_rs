//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1030/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1030<F: Float>(t2185: F, t2562: F, t1234: F, t921: F, t1553: F, t910: F, t1569: F, t938: F, t113: F, t7204: F, t6363: F, t920: F) -> (F, F, F, F, F, F, F, F) {
    let t24064 = t2562 * t2185;
    let t24070 = t921 * t1234;
    let t24118 = t910 * t1553;
    let t24161 = t24118 * t1569;
    let t24165 = t938 * t1553;
    let t24166 = t24165 * t1569;
    let t24172 = t7204 * t113;
    let t24209 = t920 * t6363;
    (t24064, t24070, t24118, t24161, t24165, t24166, t24172, t24209)
}
