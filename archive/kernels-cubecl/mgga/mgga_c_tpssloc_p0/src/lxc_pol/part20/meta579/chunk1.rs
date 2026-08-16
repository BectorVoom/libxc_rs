//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2145/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2145<F: Float>(t1020: F, t1023: F, t248: F, t43216: F, t10510: F, t3109: F, t10965: F, t3053: F, t3082: F, t3094: F, t10895: F, t10952: F) -> (F, F, F, F, F) {
    let t43219 = t1020 * t248 * t43216 * t1023;
    let t43221 = t3109 * t10510;
    let t43226 = t10965 * t3053;
    let t43228 = t3094 * t3082;
    let t43233 = t10952 * t10895;
    (t43219, t43221, t43226, t43228, t43233)
}
