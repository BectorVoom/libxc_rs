//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2140/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2140<F: Float>(t2986: F, t2990: F, t43057: F, t10325: F, t2987: F, t3008: F, t4509: F, t13797: F, t984: F, t10216: F, t343: F, t9288: F) -> (F, F, F, F, F, F) {
    let t43059 = t2986 * t43057 * t2990;
    let t43061 = t2987 * t10325;
    let t43065 = t4509 * t3008;
    let t43069 = t13797 * t984;
    let t43070 = t343 * t10216;
    let t43071 = t43070 * t9288;
    (t43059, t43061, t43065, t43069, t43070, t43071)
}
