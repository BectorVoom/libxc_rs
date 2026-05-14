//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 867/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk867<F: Float>(t2842: F, t5309: F, t684: F, t2881: F, t15191: F, t4256: F, t1091: F, t4176: F, t10703: F, t4311: F, t835: F, t1255: F, t3746: F, t4973: F, t882: F, t18123: F, t319: F) -> (F, F, F, F, F, F, F) {
    let t19585 = t2842 * t5309;
    let t19586 = t19585 * t684;
    let t19587 = t2881 * t19586;
    let t19590 = t15191 * t4256;
    let t19593 = t1091 * t4176;
    let t19594 = t10703 * t19593;
    let t19598 = t835 * t4311 * t1091;
    let t19602 = t835 * t1255 * t3746;
    let t19606 = t835 * t882 * t4973;
    let t19610 = t835 * t319 * t18123;
    (t19587, t19590, t19594, t19598, t19602, t19606, t19610)
}
