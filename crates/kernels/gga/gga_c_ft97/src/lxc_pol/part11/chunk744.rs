//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 744/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk744<F: Float>(t1934: F, t713: F, t2600: F, t2599: F, t2567: F, t2569: F, t684: F, t2606: F, t255: F, t9895: F, t2373: F, t258: F) -> (F, F, F, F, F, F, F, F) {
    let t10069 = t1934 * t713;
    let t10070 = t2600 * t10069;
    let t10071 = t2599 * t10070;
    let t10074 = t2567 * t2569;
    let t10075 = t10074 * t684;
    let t10076 = t2606 * t10075;
    let t10079 = t9895 * t255;
    let t10080 = t258 * t2373;
    (t10069, t10070, t10071, t10074, t10075, t10076, t10079, t10080)
}
