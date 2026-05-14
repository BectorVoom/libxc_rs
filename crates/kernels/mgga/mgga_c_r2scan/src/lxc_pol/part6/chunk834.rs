//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 834/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk834<F: Float>(t1946: F, t5960: F, t1762: F, t1767: F, t1987: F, t424: F, t625: F) -> (F, F, F, F, F) {
    let t5961 = t5960 * t1946;
    let t5963 = 0.28895839882605942646e1 * t1762 * t5961;
    let t5964 = t1767 * t1987;
    let t5966 = 0.96319466275353142157e0 * t1762 * t5964;
    let t5967 = t424 * t625;
    (t5961, t5963, t5964, t5966, t5967)
}
