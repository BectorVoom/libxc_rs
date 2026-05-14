//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1311/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1311<F: Float>(t1253: F, t7021: F, t15128: F, t28854: F, t10688: F, t31835: F, t1248: F, t2843: F, t28924: F, t28859: F, t4299: F, t1466: F, t31687: F, t681: F, t2749: F, t31640: F) -> (F, F, F, F, F, F, F) {
    let t125658 = t7021 * t1253;
    let t125663 = t15128 * t28854;
    let t125665 = t10688 * t31835;
    let t125668 = t2843 * t28924 * t1248;
    let t125670 = t28859 * t4299;
    let t125682 = t1466 * t681 * t31687;
    let t125684 = t2749 * t31640;
    (t125658, t125663, t125665, t125668, t125670, t125682, t125684)
}
