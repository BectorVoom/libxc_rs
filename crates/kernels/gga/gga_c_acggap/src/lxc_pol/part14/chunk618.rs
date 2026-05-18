//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 618/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk618<F: Float>(t1181: F, t1182: F, t5862: F, t1838: F, t435: F, t1165: F, t1188: F, t407: F, t1772: F, t301: F, t1089: F, t368: F) -> (F, F, F, F, F) {
    let t5864 = t1181 * t5862 * t1182;
    let t5867 = t435 * t1838;
    let t5869 = t1165 * t5867 * t1188;
    let t5873 = t1165 * t5862 * t407;
    let t5876 = t1772 * t301;
    let t5878 = t1089 * t368 * t5876;
    (t5864, t5869, t5873, t5876, t5878)
}
