//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1170/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1170<F: Float>(t10673: F, t11591: F, t37505: F, t10935: F, t2810: F, t3446: F, t11563: F, t2312: F, t3447: F, t158: F, t2461: F, t874: F) -> (F, F, F, F, F) {
    let t40428 = t10673 * t11591 * t37505;
    let t40434 = t3446 * t10935 * t2810;
    let t40451 = t3446 * t3447 * t11563 * t2312;
    let t40453 = t158 * t2461;
    let t40456 = t3446 * t3447 * t40453 * t874;
    (t40428, t40434, t40451, t40453, t40456)
}
