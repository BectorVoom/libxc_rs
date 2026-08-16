//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 898/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk898<F: Float>(t3821: F, t729: F, t773: F, t13672: F, t265: F, t1131: F, t2619: F, t2526: F, t762: F, t1160: F, t2567: F, t2569: F) -> (F, F, F, F, F) {
    let t13911 = t729 * t773 * t3821;
    let t13915 = t729 * t265 * t13672;
    let t13919 = t729 * t2619 * t1131;
    let t13922 = t1131 * t2526;
    let t13924 = t729 * t762 * t13922;
    let t13927 = t1160 * t2567;
    let t13928 = t13927 * t2569;
    (t13911, t13915, t13919, t13924, t13928)
}
