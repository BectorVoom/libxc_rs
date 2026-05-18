//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1247/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1247<F: Float>(t1020: F, t19638: F, t27763: F, t20314: F, t5310: F, t922: F, t19702: F, t3200: F, t92808: F, t15573: F, t29147: F, t7788: F) -> (F, F, F, F) {
    let t100355 = t1020 * t27763 * t19638;
    let t100360 = t5310 * t20314 * t922;
    let t100370 = t3200 * t92808 * t19702;
    let t100373 = t7788 * t15573 * t29147;
    (t100355, t100360, t100370, t100373)
}
